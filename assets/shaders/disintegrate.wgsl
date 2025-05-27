struct GlobalsUniform {
    time: f32,
    rw: u32,
    rh: u32,
    vw: u32,
    vh: u32,
}

// Vertex shader
struct CameraUniform {
    view_proj: mat4x4<f32>
};
@group(1) @binding(0)
var<uniform> camera: CameraUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec4<f32>,
    @location(2) tex_coords: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) tex_coords: vec2<f32>
};

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.color = model.color;
    out.tex_coords = model.tex_coords;
    out.clip_position = camera.view_proj * vec4<f32>(model.position, 1.0);
    return out;
}

// Fragment shader
@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;

@group(2) @binding(0)
var<uniform> globals: GlobalsUniform;

const NOISE = array(
    .1, .3, .124, .51, .78, .2, .51, .13,
    .4, .2, .632, .8,  .95, .6, .33, .7 
);

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let dim = textureDimensions(t_diffuse);
    let tex = textureSample(t_diffuse, s_diffuse, in.tex_coords);
    let n = u32(1000. * (in.tex_coords.x / in.tex_coords.y)) % 16;
    let val = NOISE[n];

    let ratio = step(1. - in.color.a, val);
    let r_offset = cos(3.14 * (1. / val));

    return vec4(tex.r + r_offset, tex.g, tex.b, tex.a * ratio);

}

