#[macro_use]
extern crate glium;

mod shader;
mod polar_game;

/*
#[test]
fn my_test() {
    panic!()
}
 */

fn main() {
    use glium::{DisplayBuild, Surface};
    use glium::glutin::Event::KeyboardInput;
    use glium::glutin::VirtualKeyCode;
    use glium::draw_parameters::LinearBlendingFactor;
    use glium::draw_parameters::BlendingFunction;
    use polar_game::object::Part;

    let screen_width = 800;
    let screen_height = 600;
    let display = glium::glutin::WindowBuilder::new().with_dimensions(screen_width,screen_height).build_glium().unwrap();

    #[derive(Copy, Clone)]
    struct Vertices {
        polar: [f32; 4],
        color: [f32; 4]
    }

    let game = polar_game::PolarGame::new();

    implement_vertex!(Vertices, polar, color);

    let vertices: Vec<Part>  = game.get_rendering_list();

    let shape: Vec<Vertices> = vertices.iter().map(|x| Vertices { polar: [x.radial[0], x.radial[1], x.angle[0], x.angle[1]], color: x.color}).collect();

    let vertex_buffer = glium::VertexBuffer::new(&display, shape);

    let indices = glium::index::NoIndices(glium::index::PrimitiveType::Points);
    let shader = shader::Shader::new(vec!["shaders/polar.vs", "shaders/polar.frag", "shaders/polar.geom"]);

    let program = glium::Program::from_source(&display,& shader.shaders[0], & shader.shaders[1], Some(& shader.shaders[2])).unwrap();

    let uniforms = uniform! {
        center: [0.0, 0.0],
        window: [screen_width as f32, screen_height as f32]
    };

    let mut draw_param =  glium::draw_parameters::DrawParameters::default();
    draw_param.blending_function = Some( BlendingFunction::Addition{source: LinearBlendingFactor::SourceAlpha,
                                                                    destination:  LinearBlendingFactor::OneMinusSourceAlpha});


    'main: loop {
        for item in display.poll_events() {
            match item
            {
                KeyboardInput(_, _, Some(VirtualKeyCode::Escape)) => break 'main,
                _ =>print!(""),
            }
        };

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.draw(&vertex_buffer, &indices, &program,
                    &uniforms,
                    &draw_param).unwrap();
        target.finish();

        if display.is_closed() {
            break;
        }
    }
}
