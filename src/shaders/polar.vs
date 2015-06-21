#version 330 core
in float vertex_index;
in vec4 polar_input;
in vec4 color_input;

out vec4 polar;
out vec4 color_vertex;

void main()
{
  polar = polar_input;
  color_vertex = color_input;
  //color_vertex = vec4(1.0f,1.0f,1.0f,1.0f);

  vec2 position = vec2(0.0f, 0.0f);
  int index = int(vertex_index);

  if (polar_input.w - polar_input.z <= 0.25)
    {
      float angleFirst = radians(polar_input.z * 360);
      float angleSecond = radians(polar_input.w * 360);
      float angleDiff = (angleFirst - angleSecond) / 2.0f;
      float radialLarge = polar.y / cos(angleDiff);
      switch (index)
	{
      case 0:
	position = polar.x * vec2(cos(angleFirst), sin(angleFirst));
	break;
      case 1:
	position = radialLarge * vec2(cos(angleFirst), sin(angleFirst));
	break;
      case 2:
	position = radialLarge * vec2(cos(angleSecond), sin(angleSecond));
	break;
      case 3:
	position = polar.x * vec2(cos(angleSecond), sin(angleSecond));
	break;
      default:
	break;
	}
    }
  else
    {
      switch (index)
	{
      case 0:
	position = polar.y * vec2(-1.0f, -1.0f);
	break;
      case 1:
	position = polar.y * vec2(-1.0f, 1.0f);
	break;
      case 2:
	position = polar.y * vec2(1.0f, 1.0f);
	break;
      case 3:
	position = polar.y * vec2(1.0f, -1.0f);
	break;
      default:
	break;
	}
    }

  gl_Position = vec4(position, 0.0f, 1.0f);
}
