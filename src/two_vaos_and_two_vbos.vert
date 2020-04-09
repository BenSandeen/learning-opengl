#version 330 core

layout (location = 0) in vec3 Position;
layout (location = 1) in vec3 Color;
//layout (location = 2) in vec3 Position_2;
//layout (location = 3) in vec3 Color_2;

out VS_OUTPUT {
    vec3 Color;
} OUT;

void main()
{
    gl_Position = vec4(Position, 1.0);
    OUT.Color = Color;
}