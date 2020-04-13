#version 330 core

out vec4 MyFragColor;

uniform vec4 ourColor;  // we set this variable in the OpenGL mode

void main()
{
    MyFragColor = ourColor;
}