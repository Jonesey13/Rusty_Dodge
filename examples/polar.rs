/*
Pure Polar Version of the Game.
*/
extern crate rusty_dodge;

use rusty_dodge::handler;

fn main() {
    let mut handler = handler::Handler::new("Polar");
    handler.init();
    'main: loop {
        handler.update_input();
        if handler.keys.exit{
            break 'main;
        }
        handler.update_physics();
        handler.update_rendering();
        if handler.keys.exit{
            break;
        }
    }
}
