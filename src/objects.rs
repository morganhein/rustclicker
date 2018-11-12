extern crate rand;

use rand::prelude::*;
pub use graphics::*;

pub struct Location {
    pub x: f64, //horizontal axis
    pub y: f64, //vertical axis
}

pub struct Invader {
    pub rotation: f64, //rotation of the object
    pub position: Location,
    pub velocity: [f64; 2], //velocity
    pub color: [f32; 4],
    pub health: f64, //size of the object
    pub destroy: bool //should be destroyed after rendering
}

impl Invader {
    pub fn new(size: piston::window::Size) -> Invader {
        let mut rng: rand::ThreadRng = thread_rng();
        let x = rng.gen_range(0, size.width);
        let vx = rng.gen_range(-4.0, 4.0);
        let vy = rng.gen_range(0.5, 5.0);
        Invader {
            rotation: 0.0,
            position: Location{
                x: x as f64,
                y: 0.0,
            },
            velocity: [vx, vy],
            color: [1.0, 0.0, 0.0, 1.0],
            health: rng.gen_range(0.0, 25.0),
            destroy: false,
        }
    }

    pub fn update(&mut self, dt: f64, size: piston::window::Size) {
        if self.health <= 0.0 {
            self.destroy = true;
        }
        if self.destroy {
            return;
        }
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

    pub fn draw<G>(&mut self, c: &Context, gl: &mut G) where G: Graphics {
        use graphics::*;
        let square = rectangle::square(0.0, 0.0, self.health);
        let tfbox = c.transform.trans(self.position.x, self.position.y)
            .rot_rad(self.rotation);
        rectangle(self.color, square, tfbox, gl);
    }

    pub fn take_hit(&mut self, dmg: u64) {
        if self.destroy {
            return;
        }
        let health: f64 = self.health - (dmg as f64);
        self.health = health as f64;
        if health <= 0.0 {
            self.destroy = true
        }
    }
}


