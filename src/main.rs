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
use std::cmp::Ordering;
//use piston::window::*;
mod player;
mod objects;

pub struct App {
    glyph: Glyphs,
    objects: Vec<objects::Invader>,
    fps: update_rate::UpdateRateCounter,
    window_size: piston::window::Size,
    player: player::Player,
}

impl App {
    fn render(&mut self, window: &mut PistonWindow, e: &piston_window::Event, args: &RenderArgs) {
        if self.window_size.height != window.size().height || self.window_size.width != window.size().width {
            println!("Window resized to height: {}, width: {}", window.size().height, window.size().width);
            self.window_size = window.size();
        };
        use graphics::*;

        const WHITE: [f32; 4] = [0.0, 0.0, 0.0, 0.0];

        self.window_size = window.size();
        window.draw_2d(e, |c, gl| {
            // Clear the screen.
            clear(WHITE, gl);

            for object in self.objects.iter_mut() {
                object.draw(&c, gl);
            };

            let tftext = c.transform.trans(450.0, 40.0);
            text::Text::new_color([0.0, 1.0, 0.0, 1.0], 32).draw(
                &format!("Points: {}", self.player.points),
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
        let mut rng: rand::ThreadRng = thread_rng();
        let make_new: i32 = rng.gen_range(0, 300);
        if make_new <= 40 {
            self.add_object();
        }
        for  object in self.objects.iter_mut() {
            object.update(args.dt, self.window_size)
        };
        //remove the shit that hit the ground
        self.objects.retain(|ref o| !o.destroy );
    }

    fn mouse_click(&mut self) {
        self.player.clicks += 1;
        self.objects.sort_unstable_by(|a, b| {
            if a.position.y == b.position.y {
                return Ordering::Equal;
            }
            if a.position.y > b.position.y {
                return Ordering::Less;
            }
            return Ordering::Greater;
        });
        let mut dmg: i64 = self.player.get_click_points() as i64;
        self.player.add_points(dmg as u64);
        let mut index: usize = 0;
        while dmg > 0 {
            if index >= self.objects.len() {
                return;
            }
            let o = &mut self.objects[index];
            o.take_hit(25);
            dmg -= 25;
            index += 1;
        }
    }


    fn add_object(&mut self) {
        use objects::*;
        let o = Invader::new(self.window_size);
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
        glyph: glyphs,
        objects: Vec::new(),
        window_size,
        fps: UpdateRateCounter::new(60),
        player: player::Player::new(),
    };

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
	    	app.mouse_click()
        }
    }
}
