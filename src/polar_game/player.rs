use polar_game::object::{Part,Object};


pub struct Player{
    pub position: [f32; 2],
    pub parts: Vec<Part>
}



impl Object for Player{
    fn set_position(&mut self, new_pos: [f32;2]){
        self.position = new_pos;
    }

    fn get_parts(&self) -> Vec<Part>{
        let mut part_vec: Vec<Part> = Vec::new();
        for p in self.parts.iter(){
            let p_shift = Part{
                radial: [p.radial[0] + self.position[0], p.radial[1] + self.position[0]],
                angle: [p.angle[0] + self.position[1], p.angle[1] + self.position[1]],
                color: p.color
            };
            part_vec.push(p_shift);
        }
        part_vec
    }
}

impl Player{
    pub fn new() -> Player{
        let prts = vec![Part{radial: [0.0, 0.1],
                          angle: [0.0, 0.1],
                              color: [1.0,1.0,1.0,1.0]}];
        Player{position: [0.5,0.0], parts: prts}
    }
}
