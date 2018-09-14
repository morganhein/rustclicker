extern crate piston;
extern crate graphics;
extern crate find_folder;
extern crate piston_window;
extern crate update_rate;
extern crate rand;

use piston::input::*;
use piston_window::*;
use update_rate::UpdateRateCounter;
use rand::prelude::*;
//use piston::window::*;

mod objects;

pub struct App {
    rotation: f64,   // Rotation for the square.
    height: i32,    // original height
    // x: i32,			// horizontal axis, not used atm
    y: i32,			// vertical axis
    glyph: Glyphs,
    points: i32,		//current points
    objects: Vec<objects::Object>,
    fps: update_rate::UpdateRateCounter,
    window_size: piston::window::Size,
    rng: rand::ThreadRng,
}

impl App {
    fn render(&mut self, window: &mut PistonWindow, e: &piston_window::Event, args: &RenderArgs) {
        if self.window_size.height != window.size().height || self.window_size.width != window.size().width {
            println!("Window resized to height: {}, width: {}", window.size().height, window.size().width);
            self.window_size = window.size();
        };
        use graphics::*;

        const WHITE: [f32; 4] = [0.0, 0.0, 0.0, 0.0];

        let square = rectangle::square(0.0, 0.0, 25.0);
        self.window_size = window.size();
        window.draw_2d(e, |c, gl| {
            // Clear the screen.
            clear(WHITE, gl);

            for object in self.objects.iter() {
                let tfbox = c.transform.trans(object.position.x, object.position.y)
                    .rot_rad(object.rotation);
                rectangle(object.color, square, tfbox, gl);
            };

            let tftext = c.transform.trans(450.0, 40.0);
            text::Text::new_color([0.0, 1.0, 0.0, 1.0], 32).draw(
                &format!("Points: {}", self.points),
                &mut self.glyph,
                &c.draw_state,
                tftext, gl)
                .unwrap();

            text::Text::new_color([0.0, 1.0, 0.0, 1.0], 12)
                .draw(&format!("{:.0} FPS", self.fps.rate()),
                      &mut self.glyph, &c.draw_state,
                      c.transform.trans(10.0, 12.0), // Set the position of the drawing
                      gl)
                .unwrap();
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.fps.update();
        if self.objects.len() == 0 {
            self.add_object();
        };
        let make_new = self.rng.gen_range(0, 300);
        if make_new <= 40 {
            self.add_object();
        }
        let mut to_delete: Vec<usize> = Vec::new();
        for (it, object) in self.objects.iter_mut().enumerate() {
            if object.destroy {
//                println!("Object hit the ground, queueing to be destroyed!");
                to_delete.push(it);
                continue;
            }
            object.update(args.dt, self.window_size)
        };
        //remove the shit that hit the ground
        for index in to_delete {
            self.objects.remove(index);
        }
    }

    fn add_click(&mut self) {
    	self.points += 1;
    }

    fn add_object(&mut self) {
//        println!("Addding new object.");
        let x = self.rng.gen_range(0, self.window_size.width);
        let vx = self.rng.gen_range(-4.0, 4.0);
        let vy = self.rng.gen_range(0.5, 5.0);
        let o = objects::Object{
            rotation: 0.0,
            position: objects::Location{
                x: x as f64,
                y: 0.0,
            },
            velocity: [vx, vy],
            color: [1.0, 0.0, 0.0, 1.0],
            destroy: false,
        };
        self.objects.push(o)
    }
}

fn main() {
    let window_size: piston::window::Size = piston::window::Size{
        height: 600,
        width: 600,
    };
    let mut window: PistonWindow = WindowSettings::new(
            "clicker",
            [600, 600]
        )
        .exit_on_esc(true)
        .build()
        .unwrap();

    let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
    println!("{:?}", assets);
	let ref font = assets.join("FiraSans-Regular.ttf");

    let factory = window.factory.clone();
	let glyphs = Glyphs::new(font, factory, TextureSettings::new()).unwrap();

    // Create a new game and run it.
    let mut app = App {
        rotation: 0.0,
        height: 600,
        // x: 300,
        y: 0,
        glyph: glyphs,
        points: 0,
        objects: Vec::new(),
        window_size,
        fps: UpdateRateCounter::new(60),
        rng: thread_rng(),

    };

    app.add_object();

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {

    // while let Some(e) = window.next() {
	    if let Some(a) = e.render_args() {
	    	app.render(&mut window, &e, &a);
	    }
	    if let Some(arg) = e.update_args() {
	    	app.update(&arg)
	    }
	    if let Some(Button::Mouse(_button)) = e.press_args() {
	    	app.add_click()
        }
    }
}
