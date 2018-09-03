extern crate piston;
extern crate graphics;
extern crate find_folder;
extern crate piston_window;

use piston::input::*;
use piston_window::*;

pub struct App {
    rotation: f64,   // Rotation for the square.
    height: i32,    // original height
    // x: i32,			// horizontal axis, not used atm
    y: i32,			// vertical axis
    glyph: Glyphs,
    points: i32		//current points
}

impl App {
    fn render(&mut self, window: &mut PistonWindow, e: &piston_window::Event, args: &RenderArgs) {
        use graphics::*;

        // const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0]; //not used
        const WHITE: [f32; 4] = [0.0, 0.0, 0.0, 0.0];
        const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.rotation;
        let (x, y) = ((args.width/2) as f64, self.y as f64);

        window.draw_2d(e, |c, gl| {
            // Clear the screen.
            clear(WHITE, gl);

            let tfbox = c.transform.trans(x, y)
                                       .rot_rad(rotation)
                                       .trans(-25.0, -25.0);

            // Draw a box rotating around the middle of the screen.
            rectangle(RED, square, tfbox, gl);

            let tftext = c.transform.trans(450.0, 40.0);
            text::Text::new_color([0.0, 1.0, 0.0, 1.0], 32).draw(
                &format!("Points: {}", self.points),
                &mut self.glyph,
                &c.draw_state,
                tftext, gl
			).unwrap();
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt;
        // and move downwards on screen per second
        self.y += 1;
        if self.y == self.height {
        	self.y = 0 as i32;
        }
    }

    fn add_click(&mut self) {
    	self.points += 1;
    }
}

fn main() {
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
        points: 0
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
	    	app.add_click()
        }
    }
}