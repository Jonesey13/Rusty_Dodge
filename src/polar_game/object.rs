use std::ops::*;

pub trait Object{
    fn set_position(&mut self, _: Point);
    fn get_position(&self) -> Point;
    fn get_render_parts(&self) -> Vec<Part>;
    fn get_collision_parts(&self) -> Vec<Part>{
        self.get_render_parts()
    }
    fn terminate_flag(&self, boundary: Point) -> bool{
        return self.get_position().x < boundary.x || self.get_position().x > boundary.y
    }
}

pub fn collision<S: Object, T: Object>( object_1: &S, object_2: &T) -> bool{
    let epsilon = 0.000000001;
    let mut overlap: bool = false;
    let parts_1 = object_1.get_collision_parts();
    let parts_2 = object_2.get_collision_parts();
    'outer: for p_1 in parts_1.iter(){
        'inner: for p_2 in parts_2.iter(){
            if p_1.radial.y < p_2.radial.x || p_1.radial.x > p_2.radial.y{
                continue 'inner;
            }
            let angle_1: Point = p_1.angle - p_1.angle.floor();
            let angle_2: Point = p_2.angle - p_2.angle.floor();

            let is_less_1: bool = angle_1.x <= angle_1.y + epsilon;
            let is_less_2: bool = angle_2.x <= angle_2.y + epsilon;

            if is_less_1 && is_less_2{
                overlap = angle_1.y >= angle_2.x && angle_1.x <= angle_2.y;
            }

            if !is_less_1 && is_less_2{
                overlap = angle_1.y >= angle_2.x || angle_1.x <= angle_2.y;
            }

            if is_less_1 && !is_less_2{
                overlap = angle_1.y >= angle_2.x || angle_1.x <= angle_2.y;
            }

            if !is_less_1 && !is_less_2{
                overlap = true;
            }

            if overlap{
                break 'outer;
            }
        }
    }
    overlap
}


#[derive(Clone,Copy)]
pub struct Part{
    pub radial: Point,
    pub angle: Point,
    pub color: [f64;4]
}

#[derive(Clone, Copy)]
pub struct Point{
    pub x: f64,
    pub y: f64
}


impl Point{
    fn floor(&self) -> Self{
        Point{x: self.x.floor(),
              y: self.y.floor()}
    }
    pub fn mult(&self, scalar: f64) -> Point{
        Point{x: self.x * scalar,
              y: self.y * scalar}
    }
}

impl Add for Point{
    type Output = Self;
    fn add(self, rhs: Self) -> Self{
        Point{x: self.x + rhs.x,
              y: self.y + rhs.y}
    }
}


impl Sub for Point{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self{
        Point{x: self.x - rhs.x,
              y: self.y - rhs.y}
    }
}
