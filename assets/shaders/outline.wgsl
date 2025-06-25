struct GlobalsUniform {
    time: f32,
    _padding_0: f32,
    render_size: vec2<u32>,
    viewport_size: vec2<u32>,
    _padding_1: f32,
    _padding_2: f32,
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

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let dim = textureDimensions(t_diffuse);
    let tex = textureSample(t_diffuse, s_diffuse, in.tex_coords);

    let du = 1.0 / f32(dim.x);
    let dv = 1.0 / f32(dim.y);

    let o = clamp(1.0 - tex.a, 0.0, 1.0);

    var na = textureSample(t_diffuse, s_diffuse, in.tex_coords - vec2(du, 0.0)).a;
    na += textureSample(t_diffuse, s_diffuse, in.tex_coords + vec2(du, 0.0)).a;
    na += textureSample(t_diffuse, s_diffuse, in.tex_coords - vec2(0.0, dv)).a;
    na += textureSample(t_diffuse, s_diffuse, in.tex_coords + vec2(0.0, dv)).a;
    na = min(na, 1.0);

    let b = 0.5 * (0.75 + 0.25 * sin(globals.time));
    let outline_color = o * na * vec4(0.0, 0.5 * b, b, 1.0);
    return tex + outline_color;
}

