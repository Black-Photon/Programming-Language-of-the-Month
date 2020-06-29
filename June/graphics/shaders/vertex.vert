#version 450

layout (location = 0) in vec3 position;
layout (location = 1) in vec3 normal;

struct Data {
    vec3 position;
    vec3 normal;
};

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

out Data data;

void main() {
    gl_Position = projection * view * model * vec4(position, 1.0);
    vec4 pos = model * vec4(position, 1.0);
    data.position = vec3(pos/pos.w);
    data.normal =  vec3(model * vec4(normal, 1.0));
}