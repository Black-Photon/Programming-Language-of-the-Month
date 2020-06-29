#version 450

struct Data {
    vec3 position;
    vec3 normal;
};

in Data data;

out vec4 colour;

uniform vec3 light;

float clamp(float f);
vec3 clamp_v(vec3 v);

void main() {
    vec3 mat_colour = vec3(0.5, 0.1, 0.7);

    vec3 ambient = vec3(0.1) * mat_colour;

    float distance = distance(light, data.position);
    vec3 irradience = vec3(4/pow(distance, 2));
    float angle = dot(normalize(light - data.position), normalize(data.normal));
    vec3 diffuse = clamp_v(mat_colour * irradience * angle);

    colour = vec4(0.1 * ambient + 0.7 * diffuse, 1.0);
}

float clamp(float f) {
    return max(min(f, 1.0), 0.0);
}

vec3 clamp_v(vec3 v) {
    return vec3(clamp(v.x), clamp(v.y), clamp(v.z));
}