/*
Defines Basic Enemy Behaviour
*/


use polar_game::player::Player;


pub trait Enemy{
    #[allow(unused_variables)]
    fn update_position(&mut self, game_time: f64, player: &Player){}
}
