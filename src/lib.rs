use std::f64;
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;
use serde::{Serialize, Deserialize};
use serde_wasm_bindgen::{to_value, from_value};

const STD_GRAVITY: f64 = 9.8;
const STD_METER_SIZE: f64 = 50.;
const PI: f64 = 3.1415;

#[derive(Clone, Serialize, Deserialize)]
pub struct Object {
    pos_x: f64,
    pos_y: f64,
    velocity_x: f64,
    velocity_y: f64,
    radius: f64,
    mass: f64,
    restitution_coef: f64
}

#[derive(Clone, Serialize, Deserialize)]
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

fn canvas_arrow(ctx: &CanvasRenderingContext2d, from_x: f64, from_y: f64, to_x: f64, to_y: f64) {
    let headlen = 10.; // length of head in pixels
    let dx = to_x - from_x;
    let dy = to_y - from_y;
    let angle = dy.atan2(dx);
    ctx.move_to(from_x, from_y);
    ctx.line_to(to_x, to_y);
    ctx.line_to(to_x - headlen * (angle - PI / 6.).cos(), to_y - headlen * (angle - PI / 6.).sin());
    ctx.move_to(to_x, to_y);
    ctx.line_to(to_x - headlen * (angle + PI / 6.).cos(), to_y - headlen * (angle + PI / 6.).sin());
  }

/// Public methods, exported to JavaScript.
#[wasm_bindgen]
impl World {
    pub fn new() -> World {
        World{
            gravity_x: 0.,
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

    pub fn get_world(&self) -> JsValue {
        to_value(self).unwrap()
    }

    pub fn set_world(js_value: JsValue) -> World {
        from_value(js_value).unwrap()  // Deserialize the JsValue back into a World struct
    }

    pub fn apply_physic(&mut self, elapsed_time_ms: u32) {
        let elapsed_time: f64 = elapsed_time_ms as f64 / 1000.;
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
        
            if overlap > 0. {
                let correction_ratio = overlap / distance;
                obj.pos_x -= delta_x * correction_ratio;
                obj.pos_y -= delta_y * correction_ratio;
                obj_.pos_x += delta_x * correction_ratio;
                obj_.pos_y += delta_y * correction_ratio;
        
                let relative_velocity_x = obj_.velocity_x - obj.velocity_x;
                let relative_velocity_y = obj_.velocity_y - obj.velocity_y;
        
                let restitution_coef = obj.restitution_coef.max(obj_.restitution_coef);
        
                let dot_product = delta_x * relative_velocity_x + delta_y * relative_velocity_y;
                let impulse = (2. * dot_product) / (distance * (obj.mass + obj_.mass));
        
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
        WorldView{canvas_width, canvas_height, center:(0., 0.), scale: 1. }
    }

    pub fn set_view_center(&mut self, center_x: f64, center_y: f64) {
        self.center.0 = center_x;
        self.center.1 = center_y;
    }

    pub fn set_scale(&mut self, scale: f64) {
        self.scale = scale; 
    }

    pub fn draw(&self, world: &World, ctx: CanvasRenderingContext2d) {
        ctx.set_fill_style_str("white");  
        ctx.fill_rect(0., 0., 800., 800.);
        ctx.set_fill_style_str("black");  
        for obj in world.objects.iter() {
            ctx.begin_path();
            ctx.arc(
                (obj.pos_x - self.center.0 + self.canvas_width as f64/2.) as f64,
                (obj.pos_y - self.center.1 + self.canvas_height as f64/2.) as f64,
                obj.radius as f64,
                0.,
                PI * 2.
            );
            ctx.fill();
            ctx.close_path();
        }
        ctx.set_font("20px Arial");
        ctx.set_fill_style_str("black");
        ctx.fill_text(&format!("View pos X:{:.0} Y:{:.0}", self.center.0, self.center.1),0. ,20.);
        ctx.fill_text(&format!("1m -> {}px", world.meter_size),0. ,40.);
    }

    pub fn draw_vectors(&self, world: &World, ctx: CanvasRenderingContext2d, scale: f64, display_values: bool) {
        ctx.set_stroke_style_str("red");
        ctx.begin_path();
        for obj in world.objects.iter() {
            let pos_x = obj.pos_x - self.center.0 + self.canvas_width as f64/2.;
            let pos_y = obj.pos_y - self.center.1 + self.canvas_height as f64/2.;
            canvas_arrow(&ctx, pos_x, pos_y, pos_x+obj.velocity_x/scale, pos_y+obj.velocity_y/scale);
            if(display_values) {
                ctx.set_font("20px Arial");
                ctx.set_fill_style_str("red");
                ctx.fill_text(&format!("{:.2}", (obj.velocity_x*obj.velocity_x + obj.velocity_y*obj.velocity_y).sqrt()),pos_x ,pos_y);
            }
        }
        ctx.stroke();
        ctx.set_stroke_style_str("green");
        ctx.begin_path();
        for obj in world.objects.iter() {
            let pos_x = obj.pos_x - self.center.0 + self.canvas_width as f64/2.;
            let pos_y = obj.pos_y - self.center.1 + self.canvas_height as f64/2.;
            canvas_arrow(&ctx, pos_x, pos_y, pos_x+world.gravity_x*5./scale, pos_y+world.gravity_y*5./scale);
            if(display_values) {
                ctx.set_font("20px Arial");
                ctx.set_fill_style_str("green");
                ctx.fill_text(&format!("{:.2}", (world.gravity_x*world.gravity_x + world.gravity_y*world.gravity_y).sqrt()),pos_x ,pos_y+20.);
            }
        }
        ctx.stroke();
    }

}