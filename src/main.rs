#[macro_use]
extern crate glium;

mod shader;

#[test]
fn my_test() {
    panic!()
}

fn main() {
    use glium::{DisplayBuild, Surface};
    use glium::glutin::Event::KeyboardInput;
    use glium::glutin::VirtualKeyCode;
    let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();

    #[derive(Copy, Clone)]
    struct Vertices {
        position: [f32; 2],
    }

    implement_vertex!(Vertices, position);

    let vertices = [[-0.5, -0.5],
                    [0.5, -0.5],
                    [0.5, 0.5],
                    [-0.5, 0.5]];
    let shape: Vec<_> = vertices.iter().map(|&x| Vertices { position: x }).collect();
    let vertex_buffer = glium::VertexBuffer::new(&display, shape);

    let ind: Vec<u32> = vec![0,1,2,0,2,3];
    let indices = glium::IndexBuffer::new(&display,glium::index::PrimitiveType::TrianglesList, ind);
    let shader = shader::Shader::new("shaders/shader.vs", "shaders/shader.frag");

    let program = glium::Program::from_source(&display, & shader.vertex_shader, & shader.fragment_shader, None).unwrap();

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
                    &glium::uniforms::EmptyUniforms,
                    &Default::default()).unwrap();
        target.finish();

        if display.is_closed() {
            break;
        }
    }
}
