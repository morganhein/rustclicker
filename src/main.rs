extern crate amethyst;

use amethyst::prelude::*;
use amethyst::renderer::{DisplayConfig, DrawFlat, Event, KeyboardInput,
                         Pipeline, PosTex, RenderBundle, Stage,
                         VirtualKeyCode, WindowEvent};
mod components;


struct GameState;

impl<'a,'b> State<GameData<'a,'b>> for GameState {
    fn on_start(&mut self, data: StateData<GameData<'a,'b>>) {
        let StateData { world, .. } = data;
        world.register::<components::Shape>();
        // let my_entity = 
        world.create_entity().with(components::Shape::Sphere{radius: 20.0, rotation: 1.0}).build();
        world.create_entity().with(components::Shape::Sphere{radius: 20.0, rotation: 1.0}).build();
        println!("Starting game!");
    }
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());
    let app_root = application_root_dir();
    let path = format!("{}/config.ron", app_root);
    let config = DisplayConfig::load(&path);
    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
            .with_pass(DrawFlat::<PosTex>::new()),
    );
    let game_data = GameDataBuilder::default().with_bundle(RenderBundle::new(pipe, Some(config)))?;    
    let mut game = Application::new("./", GameState, game_data)
        .expect("Fatal error starting application");
    game.run();
    Ok(())
}

use std::{env, fs};

/// Returns the cargo manifest directory when running the executable with cargo
/// or the directory in which the executable resides otherwise,
/// traversing symlinks if necessary.
fn application_root_dir() -> String {
    env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| {
        let mut path = env::current_exe().expect("Failed to find executable path.");
        while let Ok(target) = fs::read_link(path.clone()) {
            path = target;
        }
        String::from(
            path.parent()
                .expect("Failed to get parent directory of the executable.")
                .to_str()
                .expect("Failed to get valid string from path to the executable."),
        )
    })
}