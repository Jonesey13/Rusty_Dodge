/*
Handles the Sun Enemy Type
*/


use polar_game::object::{Part,Object,Point};
use polar_game::enemy::Enemy;

#[derive(Copy,Clone)]
pub struct Sun{
    part: Part,
}

impl Object for Sun{
    #[allow(unused_variables)]
    fn set_position(&mut self, new_pos: Point){}

    fn get_position(&self) -> Point{
        Point{x: 0.0, y:0.0}
    }

    fn get_render_parts(&self) -> Vec<Part>{
        let mut part_vec: Vec<Part> = Vec::new();
        part_vec.push(self.part);
        part_vec
    }
}

impl Sun{
    pub fn new(size: f64) -> Sun{
        let part = Part{radial: Point{x: 0.0, y: size},
                        angle: Point{x: 0.0, y: 1.0},
                        color: [0.9, 0.5, 0.2, 1.0]};
        Sun{part: part}
    }
}

impl Enemy for Sun{}
