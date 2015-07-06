use glium;
use glium::backend::glutin_backend::GlutinFacade;
use glium::glutin::Event::KeyboardInput;
use glium::glutin::VirtualKeyCode;
use glium::{DisplayBuild, Surface};
use glium::draw_parameters::LinearBlendingFactor;
use glium::draw_parameters::BlendingFunction;
use glium::glutin::ElementState::Pressed;
use glium_text;
use std::fs::File;
use std::path::Path;

use time;
use shader;
use polar_game;
use polar_game::object::{Part, Point};
use polar_game::GameSetup;

pub struct Handler<'a>{
    pub display: GlutinFacade,
    game: polar_game::PolarGame,
    pub vertex_buffer: glium::VertexBuffer<Vertices>,
    pub keys: GliumKeys,
    pub program: glium::Program,
    pub draw_param: glium::draw_parameters::DrawParameters<'a>,
    pub res: Res,
    pub radial_shift: f64,
    pub game_setup: GameSetup,
    pub txt_system: glium_text::TextSystem,
    pub font: glium_text::FontTexture,
    pub survival_start_time: f64,
}

impl<'a> Handler<'a>{

    pub fn new() -> Handler<'a>{
        let screen_width = 1024;
        let screen_height = 1024;
        let display = glium::glutin::WindowBuilder::new().with_dimensions(screen_width,screen_height).build_glium().unwrap();

        implement_vertex!(Vertices, polar, color);


        let shader = shader::Shader::new(vec!["shaders/polar.vs", "shaders/polar.frag", "shaders/polar.geom"]);
        let program = glium::Program::from_source(&display, &shader.shaders[0], &shader.shaders[1], Some(&shader.shaders[2])).unwrap();

        let mut draw_param =  glium::draw_parameters::DrawParameters::default();
        draw_param.blending_function = Some( BlendingFunction::Addition{source: LinearBlendingFactor::SourceAlpha,
                                                                        destination:  LinearBlendingFactor::OneMinusSourceAlpha});
        let game_setup = GameSetup{radial_max: 2.0,
                                   player_start: Point{x: 0.0, y: 0.75},
                                   player_width: Point{x: 0.02, y: 0.01}};

        let txt_system = glium_text::TextSystem::new(&display);

        let font = glium_text::FontTexture::new(&display, File::open(&Path::new("OpenSans.ttf")).unwrap(), 120).unwrap();

        Handler{
            vertex_buffer: glium::VertexBuffer::empty(&display, 0),
            display: display,
            game: polar_game::PolarGame::new(game_setup),
            keys: GliumKeys::new(),
            program: program,
            draw_param: draw_param,
            res: Res{width: screen_width, height: screen_height},
            radial_shift: 0.0,
            game_setup: game_setup,
            txt_system: txt_system,
            font: font,
            survival_start_time: 0.0,
        }
    }

    pub fn init(&mut self){
        self.game.init(time::precise_time_s());
        self.survival_start_time = time::precise_time_s();
    }

    pub fn update_rendering(&mut self){

        let uniforms = uniform! {
            radial_shift: self.radial_shift as f32,
            center: [0.0, 0.0],
            window: [self.res.width as f32, self.res.height as f32]
        };

        let vertices: Vec<Part>  = self.game.get_rendering_list();
        let shape: Vec<Vertices> = vertices.iter().map(|p| Vertices { polar: [p.radial.x, p.radial.y, p.angle.x, p.angle.y],
                                                                      color: p.color}).collect();
        self.vertex_buffer = glium::VertexBuffer::dynamic(&self.display, shape);

        let mut target = self.display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.draw(&self.vertex_buffer,
                    &glium::index::NoIndices(glium::index::PrimitiveType::Points),
                    &self.program,
                    &uniforms,
                    &self.draw_param)
            .unwrap();

        let time_elapsed = time::precise_time_s() - self.survival_start_time;
        let mut text_string = "Survival Time: ".to_string();
        let mut num_string = time_elapsed.to_string();
        if num_string.len() > 4{
            num_string.truncate(4);
        }
        text_string.push_str(&num_string);
        let text = glium_text::TextDisplay::new(&self.txt_system, &self.font, &text_string);

        let matrix = [[0.05, 0.0, 0.0, 0.0],
                      [0.0, 0.05, 0.0, 0.0],
                      [0.0, 0.0, 1.0, 0.0],
                      [0.3, 0.9, 0.0, 1.0]];

        glium_text::draw(&text, &self.txt_system, &mut target, matrix, (1.0, 1.0, 0.0, 1.0));

        target.finish().unwrap();
    }

    pub fn update_input(&mut self){
        let keys = &mut self.keys;
        for item in self.display.poll_events() {
            match item
            {
                KeyboardInput(_, _, Some(VirtualKeyCode::Escape)) => keys.exit = true,
                KeyboardInput(state, _, Some(VirtualKeyCode::Left)) => keys.left = state==Pressed,
                KeyboardInput(state, _, Some(VirtualKeyCode::Right)) => keys.right = state==Pressed,
                KeyboardInput(state, _, Some(VirtualKeyCode::Up)) => keys.up = state==Pressed,
                KeyboardInput(state, _, Some(VirtualKeyCode::Down)) => keys.down = state==Pressed,
                _ =>print!(""),
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
            self.game.input_keys.jump_radial = 0.2;
        }
        else if keys.down{
            self.game.input_keys.jump_radial = -0.2;
        }
        else{
            self.game.input_keys.jump_radial = -0.0;
        }

    }

    pub fn update_physics(&mut self){
        let current_time = self.game.current_time;
        let game_time = time::precise_time_s();
        let time_diff = game_time - current_time;
        self.game.update_physics(game_time);
        if self.game.player.position.x > 0.75 && self.game.player.position.x < self.game_setup.radial_max - self.game_setup.player_width.x {
            self.radial_shift = (self.radial_shift + time_diff * self.game.input_keys.jump_radial).max(0.0);
            }
    }

    pub fn reset_game(&mut self){
        self.game = polar_game::PolarGame::new(self.game_setup);
        self.keys = GliumKeys::new();
        self.radial_shift = 0.0;
        self.survival_start_time = 0.0;
    }
}





pub struct GliumKeys{
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
    pub exit: bool
}

impl GliumKeys{
    pub fn new() -> GliumKeys{
        GliumKeys{
            left: false,
            right: false,
            up: false,
            down: false,
            exit: false
        }
    }
}

#[derive(Copy, Clone)]
pub struct Vertices {
    polar: [f64; 4],
    color: [f64; 4]
}

pub struct Res{
    width: u32,
    height: u32
}
