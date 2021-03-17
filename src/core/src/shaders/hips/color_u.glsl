//const int MAX_NUM_TEX = 3;
uniform usampler2D tex1;
uniform usampler2D tex2;
uniform usampler2D tex3;
uniform int num_tex;

uniform float scale;
uniform float offset;
uniform float blank;

uniform float min_value;
uniform float max_value;
uniform int H;

uniform float size_tile_uv;

uniform int tex_storing_fits;

@import ../colormaps/colormap;
@import ./transfer_funcs;

float get_pixels(vec3 uv) {
    int idx_texture = int(uv.z);
    if (idx_texture == 0) {
        return float(texture(tex1, uv.xy).r);
    } else if (idx_texture == 1) {
        return float(texture(tex2, uv.xy).r);
    } else if (idx_texture == 2) {
        return float(texture(tex3, uv.xy).r);
    } else {
        return 0.0;
    }
}

vec3 reverse_uv(vec3 uv) {
    uv.y = size_tile_uv + 2.0*size_tile_uv*floor(uv.y / size_tile_uv) - uv.y;

    return uv;
}

uniform vec4 blank_color;

vec4 get_colormap_from_grayscale_texture(vec3 UV) {
    vec3 uv = UV;
    // FITS data pixels are reversed along the y axis
    if (tex_storing_fits == 1) {
        uv = reverse_uv(uv);
    }

    float x = get_pixels(uv);
    if (x == blank) {
        return blank_color;
    } else {
        float alpha = x * scale + offset;
        float h = transfer_func(H, alpha, min_value, max_value);

        return colormap_f(h);
    }
}

uniform vec3 C;
uniform float K;
vec4 get_color_from_grayscale_texture(vec3 UV) {
    vec3 uv = UV;
    // FITS data pixels are reversed along the y axis
    if (tex_storing_fits == 1) {
        uv = reverse_uv(uv);
    }

    float x = get_pixels(uv);
    if (x == blank) {
        return blank_color;
    } else {
        float alpha = x * scale + offset;
        float h = transfer_func(H, alpha, min_value, max_value);

        return vec4(C * K * h, 1.0);
    }
}