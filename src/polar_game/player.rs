/*
Controls the Player Behaviour
*/

use polar_game::object::{Part,Object,};
use polar_game::object::{Point};
use polar_game::GameSetup;
use rand;
use rand::distributions::IndependentSample;
use rand::distributions::range::Range;
use std::f64::consts::PI;

pub struct Player{
    pub position: Point,
    parts: Vec<Part>,
    destruct_parts: Vec<Part>,
    destruct_dirs: Vec<Point>,
    pub destroyed: bool,
}

impl Object for Player{
    fn set_position(&mut self, new_pos: Point){
        self.position = new_pos;
    }

    fn get_position(&self) -> Point{
        self.position
    }

    fn get_render_parts(&self) -> Vec<Part>{
        let mut part_vec: Vec<Part> = Vec::new();
        if !self.destroyed{
            for p in self.parts.iter(){
                let p_shift = Part{
                    radial: p.radial + Point{x: self.position.x, y: self.position.x},
                    angle: p.angle + Point{x: self.position.y, y: self.position.y},
                    color: p.color
                };
                part_vec.push(p_shift);
            }
        }
        else{
            part_vec = self.destruct_parts.clone();
        }
        part_vec
    }

    fn get_collision_parts(&self) -> Vec<Part>{
        let mut parts: Vec<Part> = Vec::new();
        if !self.destroyed{
            parts = self.get_render_parts();
        }
        parts
    }
}

impl Player{
    pub fn new(start: Point, width: Point) -> Player{
        let prts = vec![Part{radial: Point{x: 0.0, y: width.x},
                          angle: Point{x: 0.0, y: width.y},
                              color: [1.0, 1.0, 1.0, 1.0]}];
        Player{position: Point{x: start.x, y: start.y},
               parts: prts,
               destruct_parts: Vec::new(),
               destruct_dirs: Vec::new(),
               destroyed: false,}
    }

    pub fn update_position(&mut self, shift: Point, mut time_passed: f64, game_setup: GameSetup){
        if !self.destroyed{
            self.position = self.position + shift.mult(time_passed);
            self.position.x = self.position.x.min(game_setup.radial_max - game_setup.player_width.x).max(0.0);
        }
        else{
            time_passed = time_passed /5.0;
            for i in 0..100{
                let radial_shift =  Point{x: self.destruct_dirs[i].x, y: self.destruct_dirs[i].x};
                let angle_shift =  Point{x: self.destruct_dirs[i].y, y: self.destruct_dirs[i].y};
                self.destruct_parts[i].radial = self.destruct_parts[i].radial + radial_shift.mult(time_passed);
                self.destruct_parts[i].angle = self.destruct_parts[i].angle + angle_shift.mult(time_passed);;
            }
        }
    }

    pub fn get_center(&self) -> Point{
        let mut center = self.position;
        for p in self.parts.iter(){
                center.x = center.x + (p.radial.y + p.radial.x) / 2.0;
                center.y = center.y + (p.angle.y + p.angle.x) / 2.0;
        }
        center
    }

    pub fn collide(&mut self){
        let mut destructs: Vec<Part> = Vec::new();
        let center = self.get_center();
        for _ in 0..100{
            destructs.push(Part{radial: Point{x: center.x - 0.002, y: center.x + 0.002},
                               angle: Point{x: center.y - 0.002, y: center.y + 0.002},
                               color: [1.0, 1.0, 1.0, 1.0]});
        }
        self.destruct_parts = destructs;

        let mut directs: Vec<Point> = Vec::new();
        let mut rng = rand::thread_rng();
        let unif = Range::new(0.0, 1.0);
        for _ in 0..100{
            let pseudo_angle = unif.ind_sample(&mut rng);
            let radial = (2.0 * PI * pseudo_angle).cos();
            let angle = (2.0 * PI * pseudo_angle).sin();
            directs.push(Point{x: radial, y: angle});
        }
        self.destruct_dirs = directs;

        self.destroyed = true;
    }
}
