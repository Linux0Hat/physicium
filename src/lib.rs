use std::f64;
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

const STD_GRAVITY: f32 = 9.8;
const STD_METER_SIZE: f32 = 50.0;

#[derive(Clone)]
pub struct Object {
    pos_x: f32,
    pos_y: f32,
    velocity_x: f32,
    velocity_y: f32,
    radius: f32,
    mass: f32,
}


#[wasm_bindgen]
pub struct World {
    gravity_x: f32,
    gravity_y: f32,
    meter_size: f32,
    objects: Vec<Object>
}

#[wasm_bindgen]
pub struct WorldView {
    canvas_width: u32,
    canvas_height: u32,
    center: (f32, f32),
    scale: f32
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

    pub fn set_gravity_x(&mut self, gravity: f32) {
        self.gravity_x = gravity;
    }

    pub fn set_gravity_y(&mut self, gravity: f32) {
        self.gravity_y = gravity;
    }

    pub fn set_meter_size(&mut self, size: f32) {
        self.meter_size = size;
    }

    pub fn add_object(
        &mut self, pos_x: f32, pos_y: f32,
        velocity_x: f32, velocity_y: f32,
        radius: f32, mass: f32
    ) {
        self.objects.push(
            Object{pos_x, pos_y, velocity_x, velocity_y, radius, mass});
    }

    pub fn get_nb_object(&self) -> usize {
        self.objects.len()
    }

    pub fn apply_physic(&mut self, elapsed_time_ms: u32) {
        let elapsed_time: f32 = elapsed_time_ms as f32 / 1000.0;
        for obj in self.objects.iter_mut() {
            obj.velocity_x += self.gravity_x * elapsed_time * self.meter_size;
            obj.velocity_y += self.gravity_y * elapsed_time * self.meter_size;
        }
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

    pub fn set_scale(&mut self, scale: f32) {
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