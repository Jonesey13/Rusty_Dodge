use std::io::prelude::*;
use std::fs::File;
pub struct Shader{
    pub shaders: Vec<String>
}

impl Shader{
    pub fn new(shaders: Vec<&'static str>)->Shader{

        let mut strings :Vec<String> = Vec::new();
        for shad in &shaders{
            let mut shader_file = match File::open(shad){
                Ok(file) => file,
                Err(_) => panic!("Failed to Load Shader {}", shad)
            };
            let mut string_shader = String::new();
            shader_file.read_to_string(& mut string_shader).unwrap();
            strings.push(string_shader.clone());
            }
        Shader{
            shaders: strings
        }
    }

}
