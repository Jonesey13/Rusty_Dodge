use shader;
use polar_game;
use glium;
use glium::backend::glutin_backend::GlutinFacade;
use glium::glutin::Event::KeyboardInput;
use glium::glutin::VirtualKeyCode;
use glium::{DisplayBuild, Surface};
use glium::draw_parameters::LinearBlendingFactor;
use glium::draw_parameters::BlendingFunction;
use polar_game::object::Part;
use glium::glutin::ElementState::Pressed;
use time;


pub struct Handler<'a>{
    pub display: GlutinFacade,
    game: polar_game::PolarGame,
    pub vertex_buffer: glium::VertexBuffer<Vertices>,
    pub keys: GliumKeys,
    pub program: glium::Program,
    pub draw_param: glium::draw_parameters::DrawParameters<'a>,
    pub res: Res
}

impl<'a> Handler<'a>{

    pub fn new() -> Handler<'a>{
        let screen_width = 800;
        let screen_height = 800;
        let display = glium::glutin::WindowBuilder::new().with_vsync().with_dimensions(screen_width,screen_height).build_glium().unwrap();

        implement_vertex!(Vertices, polar, color);


        let shader = shader::Shader::new(vec!["shaders/polar.vs", "shaders/polar.frag", "shaders/polar.geom"]);
        let program = glium::Program::from_source(&display, &shader.shaders[0], &shader.shaders[1], Some(&shader.shaders[2])).unwrap();

        let mut draw_param =  glium::draw_parameters::DrawParameters::default();
        draw_param.blending_function = Some( BlendingFunction::Addition{source: LinearBlendingFactor::SourceAlpha,
                                                                        destination:  LinearBlendingFactor::OneMinusSourceAlpha});

        Handler{
            vertex_buffer: glium::VertexBuffer::empty(&display, 0),
            display: display,
            game: polar_game::PolarGame::new(),
            keys: GliumKeys::new(),
            program: program,
            draw_param: draw_param,
            res: Res{width: screen_width, height: screen_height}
        }
    }

    pub fn init(&mut self){
        self.game.init(time::precise_time_s() as f32);
    }

    pub fn update_rendering(&mut self){

        let uniforms = uniform! {
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
        target.finish();
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
            self.game.input_keys.jump_angle = 1.0;
        }
        else if keys.right{
            self.game.input_keys.jump_angle = -1.0;
        }
        else{
            self.game.input_keys.jump_angle = 0.0;
        }
        if keys.up{
            self.game.input_keys.jump_radial = 1.0;
        }
        else if keys.down{
            self.game.input_keys.jump_radial = -1.0;
        }
        else{
            self.game.input_keys.jump_radial = -0.0;
        }

    }

    pub fn update_physics(&mut self){
        self.game.update_physics(time::precise_time_s() as f32);
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
    polar: [f32; 4],
    color: [f32; 4]
}

pub struct Res{
    width: u32,
    height: u32
}
