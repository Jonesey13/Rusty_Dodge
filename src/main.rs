#[macro_use]
extern crate glium;

mod shader;
mod vertices;

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

    let screen_width = 800;
    let screen_height = 600;
    let display = glium::glutin::WindowBuilder::new().with_dimensions(screen_width,screen_height).build_glium().unwrap();

    #[derive(Copy, Clone)]
    struct Vertices {
        vertex_index: f32,
        polar_input: [f32; 4],
        color_input: [f32; 4]
    }

    implement_vertex!(Vertices, vertex_index, polar_input, color_input);

    let vertices = [[0.0, 0.25, 0.5, -0.05, 0.25, 1.0, 1.0, 1.0, 1.0],
                    [1.0, 0.25, 0.5, -0.05, 0.25, 1.0, 1.0, 1.0, 1.0],
                    [2.0, 0.25, 0.5, -0.05, 0.25, 1.0, 1.0, 1.0, 1.0],
                    [3.0, 0.25, 0.5, -0.05, 0.25, 1.0, 1.0, 1.0, 1.0]];


    let shape: Vec<_> = vertices.iter().map(|&x| Vertices { vertex_index: x[0], polar_input: [x[1], x[2], x[3], x[4]], color_input: [x[5], x[6], x[7], x[8]]}).collect();

    let vertex_buffer = glium::VertexBuffer::new(&display, shape);

    let ind: Vec<u32> = vec![0,1,2,0,2,3];
    let indices = glium::IndexBuffer::new(&display,glium::index::PrimitiveType::TrianglesList, ind);
    let shader = shader::Shader::new("shaders/polar.vs", "shaders/polar.frag");

    let program = glium::Program::from_source(&display, & shader.vertex_shader, & shader.fragment_shader, None).unwrap();

    let uniforms = uniform! {
        center: [0.0, 0.0],
        window: [screen_width as f32, screen_height as f32]
    };

    let mut draw_param =  glium::draw_parameters::DrawParameters::default();
    draw_param.blending_function = Some( BlendingFunction::Addition{source: LinearBlendingFactor::SourceAlpha, destination:  LinearBlendingFactor::OneMinusSourceAlpha});


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
