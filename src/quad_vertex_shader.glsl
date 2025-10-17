#version 330 core

layout(location = 0) in vec2 position;
layout(location = 1) in vec2 texturePosition;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

out vec2 textureCoordinates;

void main() {
  gl_Position = projection * view * model * vec4(position, 0.0, 1.0);
  textureCoordinates = texturePosition;
}