#version 330 core

in vec4 VertexColor;
out vec4 Color;

void main()
{
    Color = vec4(VertexColor.rgb, 1.0f);  // Just demonstrates swizzling; we could've just done `Color = VertexColor;`
}