// ANCHOR: all
// ANCHOR: includes
use ggez::{conf, event, Context, GameResult};
use std::path;
// ANCHOR_END: includes

// ANCHOR: struct
// This struct will hold all our game state
// For now there is nothing to be held, but we'll add
// things shortly.
struct Game {}
// ANCHOR_END: struct

// ANCHOR: trait
// This is the main event loop. ggez tells us to implement
// two things:
// - updating
// - rendering
impl event::EventHandler for Game {
    // ANCHOR: functions
    fn update(&mut self, _context: &mut Context) -> GameResult {
        // TODO: update game logic here
        Ok(())
    }
    // ANCHOR_END: functions

    fn draw(&mut self, _context: &mut Context) -> GameResult {
        // TODO: update draw here
        Ok(())
    }
}
// ANCHOR_END: trait

pub fn main() -> GameResult {
    // Create a game context and event loop
    let context_builder = ggez::ContextBuilder::new("rust_sokoban", "sokoban")
        .window_setup(conf::WindowSetup::default().title("Rust Sokoban!"))
        .window_mode(conf::WindowMode::default().dimensions(800.0, 600.0))
        .add_resource_path(path::PathBuf::from("./resources"));

    let (context, event_loop) = &mut context_builder.build()?;
    // Create the game state
    let game = &mut Game {};
    // Run the main event loop
    event::run(context, event_loop, game)
}
// ANCHOR_END: all
