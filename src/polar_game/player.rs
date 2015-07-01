use polar_game::object::{Part,Object};
use polar_game::object::{Point};

pub struct Player{
    pub position: Point,
    pub parts: Vec<Part>
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
        for p in self.parts.iter(){
            let p_shift = Part{
                radial: p.radial + Point{x: self.position.x, y: self.position.x},
                angle: p.angle + Point{x: self.position.y, y: self.position.y},
                color: p.color
            };
            part_vec.push(p_shift);
        }
        part_vec
    }
}

impl Player{
    pub fn new() -> Player{
        let prts = vec![Part{radial: Point{x: 0.0, y: 0.05},
                          angle: Point{x: 0.0, y: 0.02},
                              color: [1.0, 1.0, 1.0, 1.0]}];
        Player{position: Point{x: 0.5, y:0.0}, parts: prts}
    }
}
