#version 300 es
precision highp float;

layout (location = 0) in vec3 xyz;
//layout (location = 0) in vec2 lonlat;
layout (location = 1) in vec3 uv_start;
layout (location = 2) in vec3 uv_end;
layout (location = 3) in float time_tile_received;

out vec3 frag_uv_start;
out vec3 frag_uv_end;
out float frag_blending_factor;

// current time in ms
uniform mat4 inv_model;
uniform vec2 ndc_to_clip;
uniform float czf;
uniform float current_time;

#include ../../projection/projection.glsl;

void main() {
    //vec3 xyz = lonlat2xyz(lonlat);
    vec4 p_w = inv_model * vec4(xyz, 1.0); 
    // 3. Process the projection
    vec2 p_clip = proj(p_w.xyz);

    vec2 p_ndc = p_clip / (ndc_to_clip * czf);
    gl_Position = vec4(p_ndc, 0.0, 1.0);

    frag_uv_start = uv_start;
    frag_uv_end = uv_end;
    frag_blending_factor = min((current_time - time_tile_received) / 200.0, 1.0);
}