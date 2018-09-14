extern crate piston;
extern crate rand;

use rand::prelude::*;

pub struct Location {
    pub x: f64, //horizontal axis
    pub y: f64, //vertical axis
}
pub struct Object {
    pub rotation: f64, //rotation of the object
    pub position: Location,
    pub velocity: [f64; 2], //velocity
    pub color: [f32; 4],
    pub size: f64, //size of the object
    pub destroy: bool //should be destroyed after rendering
}

impl Object {
    pub fn new(size: piston::window::Size) -> Object {
        let mut rng: rand::ThreadRng = thread_rng();
        let x = rng.gen_range(0, size.width);
        let vx = rng.gen_range(-4.0, 4.0);
        let vy = rng.gen_range(0.5, 5.0);
        Object{
            rotation: 0.0,
            position: Location{
                x: x as f64,
                y: 0.0,
            },
            velocity: [vx, vy],
            color: [1.0, 0.0, 0.0, 1.0],
            size: f64,
            destroy: false,
        }
    }

    pub fn update(&mut self, dt: f64, size: piston::window::Size) {
       //detect if it's bouncing off the right
       if self.position.x >= size.width as f64 {
           self.velocity[0] = -self.velocity[0];
       }
       while self.position.x >= size.width as f64 {
           self.position.x -= 1.0;
       }
       //detect if it's bouncing off the left side
       if self.position.x <= 0.0 {
           self.velocity[0] = -self.velocity[0];
       }
       while self.position.x <= 0.0 {
           self.position.x += 1.0;
       }
       //detect if it's reached the ground
       if self.position.y >= size.height as f64 {
//           println!("We've reached the ground! Setting to be destroyed.");
           self.destroy = true;
       }

       self.position.x += self.velocity[0]* dt * 120.0;
       self.position.y += self.velocity[1] * dt * 120.0;
       self.rotation += 2.0 * dt;
    }
}


