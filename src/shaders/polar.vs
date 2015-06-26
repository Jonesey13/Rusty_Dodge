#version 330 core
in vec4 polar_input;
in vec4 color_input;

out vec4 polar_vertex;
out vec4 color_vertex;

void main()
{
  polar_vertex = polar_input;
  color_vertex = color_input;

  gl_Position = vec4(0.0f, 0.0f, 0.0f, 1.0f);
}
