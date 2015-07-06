use polar_game::object::{Part,Object,Point};
use polar_game::enemy::Enemy;
use polar_game::player::Player;

#[derive(Copy,Clone)]
pub struct Flare{
    position: Point,
    part: Part,
    velocity: f64,
}

impl Object for Flare{
    fn set_position(&mut self, new_pos: Point){
        self.position = new_pos;
    }

    fn get_position(&self) -> Point{
        self.position
    }

    fn get_render_parts(&self) -> Vec<Part>{
        let mut part_vec: Vec<Part> = Vec::new();
        let p_shift = Part{
            radial: self.part.radial + Point{x: self.position.x, y: self.position.x},
            angle: self.part.angle + Point{x: self.position.y, y: self.position.y},
            color: self.part.color
        };
        part_vec.push(p_shift);
        part_vec
    }
}

impl Flare{
    pub fn new(size: Point, start_angle: f64, velocity: f64) -> Flare{
        let part = Part{radial: Point{x: -size.x, y: 0.0},
                        angle: Point{x: start_angle, y: start_angle + size.y},
                        color: [0.8, 0.3, 0.0, 1.0]};
        Flare{position: Point{x: 0.0,
                              y: 0.0},
              part: part,
              velocity: velocity}
    }
}


impl Enemy for Flare{
    #[allow(unused_variables)]
    fn update_position(&mut self, game_time: f64, player: &Player){
        let current_position = self.get_position();
        let velocity = self.velocity;
        self.set_position(Point{x: current_position.x + game_time * velocity,
                                y: current_position.y});
    }
}
