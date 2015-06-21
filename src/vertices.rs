use std::slice::IterMut;

pub fn set_polar_vertices(vertices: &mut IterMut<f32>, polar: & Vec<f32>, color: & Vec<f32>){
    for i in 0..4{
        let mut v =  vertices.next().unwrap();
        *v = i as f32;
        for k in 0..4{
            v = vertices.next().unwrap();
            *v = polar[k];
        }
        for k in 0..4{
            v = vertices.next().unwrap();
            *v = color[k];
        }
    }
}


pub fn set_polar_indices(index: &mut IterMut<i32>, loops: i32){
    for i in 0..loops{
        *index.next().unwrap() = 0 + 5 * i;
        *index.next().unwrap() = 1 + 5 * i;
        *index.next().unwrap() = 2 + 5 * i;
        *index.next().unwrap() = 0 + 5 * i;
        *index.next().unwrap() = 2 + 5 * i;
        *index.next().unwrap() = 3 + 5 * i;
    }
}
