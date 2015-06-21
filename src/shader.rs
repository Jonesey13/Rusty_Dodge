use std::io::prelude::*;
use std::fs::File;
pub struct Shader{
    pub vertex_shader: String,
    pub fragment_shader: String
}

impl Shader{
    pub fn new(vertex_string: &str, fragment_string: &str)->Shader{
        let mut vs = match File::open(vertex_string){
            Ok(file) => file,
            Err(_) => panic!("Failed to Load Vertex Shader {}", vertex_string)
        };
        let mut strvs = String::new();
        vs.read_to_string(& mut strvs).unwrap();
        let mut fs = match File::open(fragment_string){
            Ok(file) => file,
            Err(_) => panic!("Failed to Load Fragment Shader {}", fragment_string)
        };
        let mut strfs = String::new();
        fs.read_to_string(& mut strfs).unwrap();
        Shader{
            vertex_shader: strvs,
            fragment_shader: strfs,
        }

    }

}
