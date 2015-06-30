#[macro_use]
extern crate glium;
extern crate time;

mod shader;
mod polar_game;
mod handler;

/*
#[test]
fn my_test() {
    panic!()
}
 */

fn main() {
    let mut handler = handler::Handler::new();
    handler.init();
    'main: loop {
        handler.update_input();
        if handler.keys.exit{
            break 'main;
        }
        handler.update_physics();
        handler.update_rendering();
        if handler.display.is_closed(){
            break;
        }
    }
}
