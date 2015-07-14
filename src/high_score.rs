use std::io::prelude::*;
use std::fs::File;
use std::string::String;

pub fn load_high_score() -> f64{
    let path = "highscore.txt".to_string();
    let mut file = match File::open(path){
        Ok(f) => f,
        Err(_) => return 0.0,
    };
    let mut score_string = String::new();
    match file.read_to_string(&mut score_string){
        Ok(r) => r,
        Err(_) => panic!("failed to read score to string"),
    };
    match score_string.parse::<f64>(){
        Ok(s) => s,
        Err(_) => panic!("failed to parse string to f64"),
    }
}

pub fn write_high_score(new_score: f64){
    let score_string = new_score.to_string();
    let path = "highscore.txt".to_string();
    let mut file = match File::create(path){
        Ok(f) => f,
        Err(_) => panic!("Failed to open High Score!"),
    };
    match file.write_all(score_string.as_bytes()){
        Ok(f) => f,
        Err(_) => panic!("failed to write sting to file"),
    };
}
