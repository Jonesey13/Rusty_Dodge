/*
Central file for the polar_game module
*/

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
use time;
use rand;
use rand::distributions::exponential::Exp;
use rand::distributions::IndependentSample;
use rand::distributions::range::Range;

pub struct PolarGame{
    player: Player,
    flares: Vec<Flare>,
    pub input_keys: InputKeys,
    frame: PolarFrame,
    pub setup: GameSetup,
    time: Times,
    pub state: GameState,
}

impl PolarGame{

    pub fn new(setup: GameSetup) -> PolarGame{
        PolarGame{
            player: Player::new(setup.player_start, setup.player_width),
            flares: Vec::new(),
            input_keys: InputKeys{
                jump_angle: 0.0,
                jump_radial: 0.0,
            },
            time: Times::new(0.0),
            frame: PolarFrame::new(20, 20, Point{x: 0.01, y: 0.02}, setup.radial_max),
            setup: setup,
            state: GameState::new(),
        }
    }

    pub fn init(&mut self){
        self.time = Times::new(time::precise_time_s());
    }

    pub fn update_physics(&mut self){
        let shift = Point{x: self.input_keys.jump_radial,
                          y: self.input_keys.jump_angle / 2.0};
        let current_time = time::precise_time_s();
        let time_diff =  current_time - self.time.elapsed;
        self.time.elapsed = current_time;

        self.player.update_position(shift, time_diff, self.setup);
        for mut f in self.flares.iter_mut(){
            f.update_position(time_diff, &self.player);
            if collision(&*f, &self.player){
                self.player.collide();;
            }
        }

        let current_flares = self.flares.clone();
        let (_, flares_trimmed) : (Vec<Flare>, Vec<Flare>)
            = current_flares.into_iter().partition(|f| f.terminate_flag(Point{x: -1.0, y: 2.0}));
        self.flares = flares_trimmed;


        if self.time.elapsed - self.time.previous_flare > self.time.til_flare{
            let mut rng = rand::thread_rng();
            let unif = Range::new(0.0, 1.0);
            let sa = unif.ind_sample(&mut rng);
            let r = unif.ind_sample(&mut rng) / 20.0 + 0.02;
            let a = unif.ind_sample(&mut rng) / 50.0 + 0.005;
            let v = unif.ind_sample(&mut rng) / 2.0 + 0.1;
            let new_flare = Flare::new(Point{x: r, y: a}, sa, v);
            self.flares.push(new_flare);
            self.time.previous_flare = self.time.elapsed;
            let emit_average = (5.0 + self.time.elapsed - self.time.start ) / 5.0 + 4.0;
            let exp = Exp::new(emit_average);
            self.time.til_flare = exp.ind_sample(&mut rng);
        }

        let mut new_survival_time = self.state.survival_time;;
        if !self.player.destroyed{
            new_survival_time = self.time.elapsed - self.time.survival_start;
        }
        self.state = GameState{player_death: self.player.destroyed,
                               survival_time: new_survival_time,
                               };
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

    pub fn get_player_position(&self) -> Point{
        self.player.get_position()
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

#[derive(Copy, Clone)]
pub struct GameState{
    pub player_death: bool,
    pub survival_time: f64,
}

impl GameState{
    pub fn new() -> GameState{
        GameState{ player_death: false,
                   survival_time: 0.0,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Times{
    til_flare: f64,
    previous_flare: f64,
    start: f64,
    survival_start: f64,
    elapsed: f64,
}

impl Times{
    pub fn new(start_time: f64) -> Times{
        let mut rng = rand::thread_rng();
        let exp = Exp::new(1.0);
        Times{ til_flare: exp.ind_sample(&mut rng),
               previous_flare: start_time,
               start: start_time,
               survival_start: start_time,
               elapsed: start_time,
        }
    }
}
