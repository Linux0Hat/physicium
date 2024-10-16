use std::f64;
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

const STD_GRAVITY: f64 = 9.8;
const STD_METER_SIZE: f64 = 50.0;

#[derive(Clone)]
pub struct Object {
    pos_x: f64,
    pos_y: f64,
    velocity_x: f64,
    velocity_y: f64,
    radius: f64,
    mass: f64,
    restitution_coef: f64
}
#[derive(Clone)]
#[wasm_bindgen]
pub struct World {
    gravity_x: f64,
    gravity_y: f64,
    meter_size: f64,
    objects: Vec<Object>
}

#[wasm_bindgen]
pub struct WorldView {
    canvas_width: u32,
    canvas_height: u32,
    center: (f64, f64),
    scale: f64
}

fn gen_pairs(n: usize) -> Vec<[usize; 2]> {
    let mut pairs = Vec::new();

    for i in 0..n {
        for j in 0..n {
            if i != j {
                pairs.push([i, j]);
            }
        }
    }

    pairs
}

/// Public methods, exported to JavaScript.
#[wasm_bindgen]
impl World {
    pub fn new() -> World {
        World{
            gravity_x: 0.0,
            gravity_y: STD_GRAVITY,
            meter_size: STD_METER_SIZE,
            objects: Vec::new()
        }
    }

    pub fn set_gravity_x(&mut self, gravity: f64) {
        self.gravity_x = gravity;
    }

    pub fn set_gravity_y(&mut self, gravity: f64) {
        self.gravity_y = gravity;
    }

    pub fn set_meter_size(&mut self, size: f64) {
        self.meter_size = size;
    }

    pub fn add_object(
        &mut self, pos_x: f64, pos_y: f64,
        velocity_x: f64, velocity_y: f64,
        radius: f64, mass: f64, restitution_coef: f64
    ) {
        self.objects.push(
            Object{pos_x, pos_y, velocity_x, velocity_y, radius, mass, restitution_coef});
    }

    pub fn get_world(&self) -> World {
        self.clone()
    }

    pub fn apply_physic(&mut self, elapsed_time_ms: u32) {
        let elapsed_time: f64 = elapsed_time_ms as f64 / 1000.0;
        let pairs = gen_pairs(self.objects.len());

        // gravity
        for obj in self.objects.iter_mut() {
            obj.velocity_x += self.gravity_x * elapsed_time * self.meter_size;
            obj.velocity_y += self.gravity_y * elapsed_time * self.meter_size;
        }

        // universal gravitation
        for pair in pairs.iter() {
            let (left, right) = self.objects.split_at_mut(pair[0].max(pair[1]));
            let obj = &mut left[pair[0].min(pair[1])];
            let obj_ = &mut right[0];
            let delta_x = obj_.pos_x - obj.pos_x;
            let delta_y = obj_.pos_y - obj.pos_y;
            let d = (delta_x * delta_x + delta_y * delta_y).sqrt() / self.meter_size;
            let G = 6.674e-11;
            let F = ((G * (obj.mass * obj_.mass)) / (d * d));
            let a = delta_y.atan2(delta_x);
            obj.velocity_x += F * elapsed_time * a.cos();
            obj.velocity_y += F * elapsed_time * a.sin();
            obj_.velocity_x -= F * elapsed_time * a.cos();
            obj_.velocity_y -= F * elapsed_time * a.sin();
        }

        // collisions
        for pair in pairs.iter() {
            let (left, right) = self.objects.split_at_mut(pair[0].max(pair[1]));
            let obj = &mut left[pair[0].min(pair[1])];
            let obj_ = &mut right[0];
        
            let delta_x = obj_.pos_x - obj.pos_x;
            let delta_y = obj_.pos_y - obj.pos_y;
            let distance = (delta_x * delta_x + delta_y * delta_y).sqrt();
            let overlap = obj.radius + obj_.radius - distance;
        
            if overlap > 0.0 {
                let a = (obj_.pos_y - obj.pos_y).atan2(obj_.pos_x - obj.pos_x);
                let correction_ratio = overlap / distance;
                obj.pos_x -= delta_x * correction_ratio;
                obj.pos_y -= delta_y * correction_ratio;
                obj_.pos_x += delta_x * correction_ratio;
                obj_.pos_y += delta_y * correction_ratio;
        
                let relative_velocity_x = obj_.velocity_x - obj.velocity_x;
                let relative_velocity_y = obj_.velocity_y - obj.velocity_y;
        
                let restitution_coef = obj.restitution_coef.max(obj_.restitution_coef);
        
                let dot_product = delta_x * relative_velocity_x + delta_y * relative_velocity_y;
                let impulse = (2.0 * dot_product) / (distance * (obj.mass + obj_.mass));
        
                obj.velocity_x += (impulse * obj_.mass * delta_x * restitution_coef) / distance;
                obj.velocity_y += (impulse * obj_.mass * delta_y * restitution_coef) / distance;
        
                obj_.velocity_x -= (impulse * obj.mass * delta_x * restitution_coef) / distance;
                obj_.velocity_y -= (impulse * obj.mass * delta_y * restitution_coef) / distance;
            }
        }

        // position
        for obj in self.objects.iter_mut() {
            obj.pos_x += obj.velocity_x * elapsed_time;
            obj.pos_y += obj.velocity_y * elapsed_time;
        }
    }
}


#[wasm_bindgen]
impl WorldView {
    pub fn new(canvas_width: u32, canvas_height: u32) -> WorldView {
        WorldView{canvas_width, canvas_height, center:(0.0, 0.0), scale: 1.0 }
    }

    pub fn set_view_center(&mut self, canvas_width: u32, canvas_height: u32) {
        self.canvas_width = canvas_width;
        self.canvas_height = canvas_height;
    }

    pub fn set_scale(&mut self, scale: f64) {
        self.scale = scale; 
    }

    pub fn draw(&self, world: &World, ctx: CanvasRenderingContext2d) {
        ctx.set_fill_style(&"white".into());  
        ctx.fill_rect(0., 0., 800., 800.);
        ctx.set_fill_style(&"black".into());  
        for obj in world.objects.iter() {
            ctx.begin_path();
            ctx.arc(
                (obj.pos_x - self.center.0) as f64,
                (obj.pos_y - self.center.1) as f64,
                obj.radius as f64,
                0.0,
                3.1415 * 2.0
            );
            // ctx.set_fill_style(&"rgb(0,0,0)".into());
            ctx.fill();
            ctx.close_path();
        }
    }

}