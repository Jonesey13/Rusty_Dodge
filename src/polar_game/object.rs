pub trait Object{
    fn set_position(&mut self, _: [f32;2]);
    fn get_parts(&self) -> Vec<Part>;
}

#[allow(dead_code)]
fn collision<T: Object>(object_1: T, object_2: T) -> bool{
    let epsilon = 0.000000001;
    let mut overlap: bool = false;
    let parts_1 = object_1.get_parts();
    let parts_2 = object_2.get_parts();
    'outer: for p_1 in parts_1.iter(){
        'inner: for p_2 in parts_2.iter(){
            if p_1.radial[1] > p_2.radial[0] && p_1.radial[0] < p_2.radial[1]{
                continue 'inner;
            }
            let angle_1: Vec<f32> = p_1.angle.to_vec().iter().map(|&x| x-x.floor()).collect();
            let angle_2: Vec<f32> = p_2.angle.to_vec().iter().map(|&x| x-x.floor()).collect();

            let is_less_1: bool = angle_1[0] <= angle_1[1] + epsilon;
            let is_less_2: bool = angle_2[0] <= angle_2[1] + epsilon;

            if is_less_1 && is_less_2{
                overlap = angle_1[1] >= angle_2[0] && angle_1[0] <= angle_2[1];
            }

            if !is_less_1 && is_less_2{
                overlap = angle_1[1] >= angle_2[0] || angle_1[0] <= angle_2[1];
            }

            if is_less_1 && !is_less_2{
                overlap = angle_1[1] >= angle_2[0] || angle_1[0] <= angle_2[1];
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

#[derive(Clone)]
pub struct Part{
    pub radial: [f32;2],
    pub angle: [f32;2],
    pub color: [f32;4]
}
