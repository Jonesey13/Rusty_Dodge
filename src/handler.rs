/*
Handles Rendering and Input for the game.
The games itself makes no contact with glium.
*/

use glium;
use glium::backend::glutin_backend::GlutinFacade;
use glium::glutin::Event::{Closed,KeyboardInput};
use glium::glutin::VirtualKeyCode;
use glium::{DisplayBuild, Surface, Blend};
use glium::glutin::ElementState::Pressed;
use glium_text;
use std::fs::File;
use std::path::Path;
use time;

use shader;
use high_score;
use polar_game;
use polar_game::object::{Part, Point};
use polar_game::GameSetup;

pub struct Handler<'a>{
    display: GlutinFacade,
    game: polar_game::PolarGame,
    vertex_buffer: glium::VertexBuffer<Vertices>,
    pub keys: GliumKeys,
    program: glium::Program,
    draw_param: glium::draw_parameters::DrawParameters<'a>,
    radial_shift: f64,
    game_setup: GameSetup,
    txt_system: glium_text::TextSystem,
    font: glium_text::FontTexture,
    high_score: f64,
}


impl<'a> Handler<'a>{

    pub fn new(mode: &'static str) -> Handler<'a>{
        let display = // if cfg!(any(target_os = "macos", target_os = "windows")){
        //     glium::glutin::WindowBuilder::new().with_fullscreen(glium::glutin::get_primary_monitor()).build_glium().unwrap()
        // }
        // else{
            glium::glutin::WindowBuilder::new().with_dimensions(800,600).build_glium().unwrap()
        ;

        implement_vertex!(Vertices, polar, color);

        let tunnel_mode = mode == "Tunnel";

        let shader = if tunnel_mode{
            shader::Shader::new(vec!["shaders/tunnel.vs", "shaders/polar.frag", "shaders/polar.geom"])
        }
        else{
            shader::Shader::new(vec!["shaders/polar.vs", "shaders/polar.frag", "shaders/polar.geom"])
        };
        let program = glium::Program::from_source(&display, &shader.shaders[0], &shader.shaders[1], Some(&shader.shaders[2])).unwrap();

        let mut draw_param =  glium::draw_parameters::DrawParameters::default();
        draw_param.blend = Blend::alpha_blending();
        let radial_max = 8.0;
        let game_setup = GameSetup{radial_max: radial_max,
                                   player_start: Point{x: radial_max - 4.0, y: 0.75},
                                   player_width: Point{x: 0.02, y: 0.01}};



        let txt_system = glium_text::TextSystem::new(&display);
        let font_file = match File::open(&Path::new("OpenSans.ttf")){
            Ok(file) => file,
            Err(_) => match File::open(&Path::new("src/OpenSans.ttf")){
                Ok(file) => file,
                Err(_) => match File::open(&Path::new("../OpenSans.ttf")){
                    Ok(file) => file,
                    Err(_) => panic!("Font Failed to Load"),
                    }
                }
            };

        let font = glium_text::FontTexture::new(&display, font_file, 120).unwrap();
        let high_score = high_score::load_high_score();

        Handler{
            vertex_buffer: glium::VertexBuffer::empty(&display, 0).unwrap(),
            display: display,
            game: polar_game::PolarGame::new(game_setup),
            keys: GliumKeys::new(),
            program: program,
            draw_param: draw_param,
            radial_shift: 0.0,
            game_setup: game_setup,
            txt_system: txt_system,
            font: font,
            high_score: high_score,
        }
    }

    pub fn init(&mut self){
        self.game.init();
    }

    pub fn update_rendering(&mut self){
        let (w, h) = self.display.get_framebuffer_dimensions();
        let aspect_ratio = (w as f32) / (h as f32);
        let uniforms = uniform! {
            // Tunnel Version Exclusive
            length_total: (self.game.get_player_position().x + 0.25).max(1.0) as f32,
            length_circle: 1.0 as f32,
            // Polar Version Exclusive
            radial_shift: self.radial_shift as f32,
            // General
            center: [0.0 as f32, 0.0 as f32],
            aspect_ratio: aspect_ratio,
        };

        let vertices: Vec<Part>  = self.game.get_rendering_list();
        let shape: Vec<Vertices> = vertices.iter().map(|p| Vertices { polar: [p.radial.x, p.radial.y, p.angle.x, p.angle.y],
                                                                      color: p.color}).collect();
        self.vertex_buffer = glium::VertexBuffer::dynamic(&self.display, &shape).unwrap();

        let pre_time = time::precise_time_s();
        let mut target = self.display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.draw(&self.vertex_buffer,
                    &glium::index::NoIndices(glium::index::PrimitiveType::Points),
                    &self.program,
                    &uniforms,
                    &self.draw_param)
            .unwrap();

        let mut survival_string = "Survival Time: ".to_string();
        let survival_time = self.game.state.survival_time;
        if survival_time > self.high_score{
            self.high_score = survival_time;
        }
        let mut num_string = survival_time.to_string();
        if num_string.len() > 5{
            num_string.truncate(5);
        }
        survival_string.push_str(&num_string);

        let mut high_score_string = "High Score: ".to_string();
        num_string = self.high_score.to_string();
        if num_string.len() > 5{
            num_string.truncate(5);
        }
        high_score_string.push_str(&num_string);

        let survival_text = glium_text::TextDisplay::new(&self.txt_system, &self.font, &survival_string);
        let high_score_text = glium_text::TextDisplay::new(&self.txt_system, &self.font, &high_score_string);

        let matrix = [[0.05 / aspect_ratio, 0.0, 0.0, 0.0],
                      [0.0, 0.05, 0.0, 0.0],
                      [0.0, 0.0, 1.0, 0.0],
                      [0.3, 0.9, 0.0, 1.0]];

        glium_text::draw(&survival_text, &self.txt_system, &mut target, matrix, (1.0, 1.0, 0.0, 1.0));

        let matrix = [[0.05 / aspect_ratio, 0.0, 0.0, 0.0],
                      [0.0, 0.05, 0.0, 0.0],
                      [0.0, 0.0, 1.0, 0.0],
                      [0.3, 0.8, 0.0, 1.0]];

        glium_text::draw(&high_score_text, &self.txt_system, &mut target, matrix, (1.0, 1.0, 0.0, 1.0));

        target.finish().unwrap();
        let after_time = time::precise_time_s();
        println!("{}", after_time - pre_time);
    }

    pub fn update_input(&mut self){
        let keys = &mut self.keys;
        for item in self.display.poll_events() {
            match item
            {
                Closed => keys.exit = true,
                KeyboardInput(state, _, Some(VirtualKeyCode::Escape)) => keys.exit = state == Pressed,
                KeyboardInput(state, _, Some(VirtualKeyCode::Return)) => keys.reset = state == Pressed,
                KeyboardInput(state, _, Some(VirtualKeyCode::Left)) => keys.left = state == Pressed,
                KeyboardInput(state, _, Some(VirtualKeyCode::Right)) => keys.right = state == Pressed,
                KeyboardInput(state, _, Some(VirtualKeyCode::Up)) => keys.up = state == Pressed,
                KeyboardInput(state, _, Some(VirtualKeyCode::Down)) => keys.down = state == Pressed,
                _ => (),
            }
        }
        if keys.left{
            self.game.input_keys.jump_angle = -0.5;
        }
        else if keys.right{
            self.game.input_keys.jump_angle = 0.5;
        }
        else{
            self.game.input_keys.jump_angle = 0.0;
        }
        if keys.up{
            self.game.input_keys.jump_radial = -0.2;
        }
        else if keys.down{
            self.game.input_keys.jump_radial = 0.2;
        }
        else{
            self.game.input_keys.jump_radial = -0.0;
        }
        if keys.exit{
            high_score::write_high_score(self.high_score);
        }
    }

    pub fn update_physics(&mut self){
        self.game.update_physics();
        if self.keys.reset{
            self.reset_game();
        }
        let player_position = self.game.get_player_position();
        if player_position.x > 0.75 {
            self.radial_shift = player_position.x - 0.75;
        }
        else{
            self.radial_shift = 0.0;
        }
    }

    pub fn reset_game(&mut self){
        self.game = polar_game::PolarGame::new(self.game_setup);
        self.keys = GliumKeys::new();
        self.radial_shift = 0.0;
        self.init();
    }
}





pub struct GliumKeys{
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
    pub exit: bool,
    pub reset: bool,
}

impl GliumKeys{
    pub fn new() -> GliumKeys{
        GliumKeys{
            left: false,
            right: false,
            up: false,
            down: false,
            exit: false,
            reset: false,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Vertices {
    polar: [f64; 4],
    color: [f64; 4]
}
