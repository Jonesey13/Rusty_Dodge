
pub mod player;
pub mod object;
use polar_game::player::Player;
use polar_game::object::{Part,Object,Point};

pub struct PolarGame{
    player: Player,
    current_time: f32,
    pub input_keys: InputKeys
}



impl PolarGame{

    pub fn new() -> PolarGame{
        PolarGame{
            player: Player::new(),
            current_time: 0.0,
            input_keys: InputKeys{
                jump_angle: 0.0,
                jump_radial: 0.0
            }
        }
    }

    pub fn init(&self, game_time: f32){

    }

    pub fn update_input(&self){


    }

    pub fn update_physics(&mut self, game_time: f32){
        let shift = Point{x: self.input_keys.jump_radial,
                          y: self.input_keys.jump_angle};
        self.player.position =  self.player.position + shift.mult(game_time/100.0);
    }

    pub fn get_rendering_list(&self) -> Vec<Part>{
        self.player.get_parts()
    }

}

pub struct InputKeys{
    pub jump_angle: f32,
    pub jump_radial: f32
}

impl InputKeys{

    pub fn new() -> InputKeys{
        InputKeys{jump_angle: 0.0,
                  jump_radial: 0.0}
    }

}
