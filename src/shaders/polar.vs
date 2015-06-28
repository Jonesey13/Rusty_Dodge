#version 330 core
in vec4 polar;
in vec4 color;

out vec4 polar_vertex;
out vec4 color_vertex;

void main()
{
  polar_vertex = polar;
  color_vertex = color;

  gl_Position = vec4(0.0f, 0.0f, 0.0f, 1.0f);
}
