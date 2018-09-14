extern crate piston;


pub struct Location {
    pub x: f64, //horizontal axis
    pub y: f64, //vertical axis
}
pub struct Object {
    pub rotation: f64, //rotation of the object
    pub position: Location,
    pub velocity: [f64; 2], //velocity
    pub color: [f32; 4],
    pub destroy: bool //should be destroyed after rendering
}

impl Object {
   pub fn update(&mut self, dt: f64, size: piston::window::Size) {
       //detect if it's bouncing off the right
       if self.position.x >= size.width as f64 {
           self.velocity[0] = -self.velocity[0];
       }
       //detect if it's bouncing off the left side
       if self.position.x <= 0.0 {
           self.velocity[0] = -self.velocity[0];
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


