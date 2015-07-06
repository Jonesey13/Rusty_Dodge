pub mod player;
pub mod object;
pub mod enemy;
pub mod flare;
pub mod frame;
use polar_game::player::Player;
use polar_game::object::{Part,Object,Point,collision};
use polar_game::flare::Flare;
use polar_game::enemy::Enemy;
use polar_game::frame::PolarFrame;
use rand;
use rand::distributions::exponential::Exp;
use rand::distributions::IndependentSample;
use rand::distributions::range::Range;

pub struct PolarGame{
    pub player: Player,
    flares: Vec<Flare>,
    pub current_time: f64,
    time_til_flare: f64,
    previous_flare_time: f64,
    start_time: f64,
    pub input_keys: InputKeys,
    frame: PolarFrame,
    pub setup: GameSetup,
}

impl PolarGame{

    pub fn new(setup: GameSetup) -> PolarGame{
        PolarGame{
            player: Player::new(setup.player_start, setup.player_width),
            flares: Vec::new(),
            current_time: 0.0,
            input_keys: InputKeys{
                jump_angle: 0.0,
                jump_radial: 0.0,
            },
            time_til_flare: 1.0,
            previous_flare_time: 0.0,
            start_time: 0.0,
            frame: PolarFrame::new(20, 20, Point{x: 0.01, y: 0.02}, setup.radial_max),
            setup: setup,
        }
    }

    pub fn init(&mut self, game_time: f64){

        self.current_time = game_time;
        self.start_time = game_time;
        self.previous_flare_time = self.current_time;
        let mut rng = rand::thread_rng();
        let exp = Exp::new(1.0);
        self.time_til_flare = exp.ind_sample(&mut rng);
    }

    pub fn update_physics(&mut self, game_time: f64){
        let shift = Point{x: self.input_keys.jump_radial,
                          y: self.input_keys.jump_angle / 2.0};
        let time_diff = game_time - self.current_time;
        self.current_time = game_time;
        self.player.position =  self.player.position + shift.mult(time_diff);
        self.player.position.x = self.player.position.x.min(self.setup.radial_max - self.setup.player_width.x).max(0.0);
        for mut f in self.flares.iter_mut(){
            f.update_position(time_diff, &self.player);
            if collision(&*f, &self.player){
                self.player.parts[0].color = [1.0, 0.0, 0.0, 1.0];
            }
        }

        let current_flares = self.flares.clone();
        let (_, flares_trimmed) : (Vec<Flare>, Vec<Flare>)
            = current_flares.into_iter().partition(|f| f.terminate_flag(Point{x: -1.0, y: 2.0}));
        self.flares = flares_trimmed;


        if self.current_time - self.previous_flare_time > self.time_til_flare{
            let mut rng = rand::thread_rng();
            let unif = Range::new(0.0, 1.0);
            let sa = unif.ind_sample(&mut rng);
            let r = unif.ind_sample(&mut rng) / 20.0 + 0.02;
            let a = unif.ind_sample(&mut rng) / 50.0 + 0.005;
            let v = unif.ind_sample(&mut rng) / 2.0 + 0.1;
            let new_flare = Flare::new(Point{x: r, y: a}, sa, v);
            self.flares.push(new_flare);
            self.previous_flare_time = self.current_time;
            let emit_average = (5.0 + game_time - self.start_time ) / 5.0 + 4.0;
            let exp = Exp::new(emit_average);
            self.time_til_flare = exp.ind_sample(&mut rng);
        }


    }

    pub fn get_rendering_list(&self) -> Vec<Part>{
        let mut rend_vec: Vec<Part> = Vec::new();
        for f in self.frame.get_render_parts().iter(){
            rend_vec.push(f.clone());
        }
        for f in self.player.get_render_parts().iter(){
            rend_vec.push(f.clone());
        }
        for f in self.flares.iter(){
            let flare_part = f.get_render_parts()[0];
            rend_vec.push(flare_part);
        }
        rend_vec
    }

}

pub struct InputKeys{
    pub jump_angle: f64,
    pub jump_radial: f64
}

#[derive(Copy, Clone)]
pub struct GameSetup{
    pub radial_max: f64,
    pub player_start: Point,
    pub player_width: Point,
}
