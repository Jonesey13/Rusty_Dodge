
pub mod player;
pub mod object;
use polar_game::player::Player;
use polar_game::object::{Part,Object};

pub struct PolarGame{
    player: Player,
    current_time: f32,
    pub input_keys: Input_Keys
}



impl PolarGame{

    pub fn new() -> PolarGame{
        PolarGame{
            player: Player::new(),
            current_time: 0.0,
            input_keys: Input_Keys{
                jump_angle: 0.0,
                jump_radial: 0.0
            }
        }
    }

    pub fn init(&self, game_time: f32){

    }

    pub fn update_input(&self){


    }

    pub fn update_physics(&self, game_time: f32){



    }

    pub fn get_rendering_list(&self) -> Vec<Part>{
        self.player.get_parts()
    }

}

pub struct Input_Keys{
    pub jump_angle: f32,
    pub jump_radial: f32
}
