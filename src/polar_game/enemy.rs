use polar_game::player::Player;


pub trait Enemy{
    fn update_position(&mut self, game_time: f64, player: &Player);
}
