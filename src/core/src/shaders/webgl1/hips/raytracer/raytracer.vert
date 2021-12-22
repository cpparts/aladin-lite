#ifdef WEBGL2
    #version 300 es
    layout (location = 0) in vec2 pos_clip_space;
    out vec2 out_clip_pos;
#else
    attribute vec2 pos_clip_space;
    varying vec2 out_clip_pos;
#endif

precision highp float;
precision highp int;

uniform vec2 ndc_to_clip;
uniform float czf;

void main() {
    gl_Position = vec4(pos_clip_space / (ndc_to_clip * czf), 0.0, 1.0);
    out_clip_pos = pos_clip_space;
}