struct GlobalsUniform {
    time: f32,
    rw: u32,
    rh: u32,
    vw: u32,
    vh: u32,
}

struct Uniform {
    strength: f32,
    _padding_0: f32,
    _padding_1: f32,
    _padding_2: f32,
}

struct VertexOutput {
    @location(0) uv: vec2<f32>,
    @builtin(position) clip_position: vec4<f32>,
}

@vertex
fn vs_main(
    @builtin(vertex_index) vi: u32
) -> VertexOutput {
    var out: VertexOutput;
    out.uv = vec2<f32>(
        f32((vi << 1u) & 2u),
        f32(vi & 2u),
    );
    out.clip_position = vec4<f32>(out.uv * 2.0 - 1.0, 0.0, 1.0);
    out.uv.y = 1.0 - out.uv.y;
    return out;
}

@group(0)
@binding(0)
var input_image: texture_2d<f32>;

@group(0)
@binding(1)
var input_sampler: sampler;

@group(0)
@binding(4)
var<uniform> uniform: Uniform;

@group(1) @binding(0)
var<uniform> globals: GlobalsUniform;

@fragment
fn fs_main(vs: VertexOutput) -> @location(0) vec4<f32> {
    let col = textureSample(input_image, input_sampler, vs.uv);

    let y = vs.uv.y * f32(globals.vh);
    return 1.25 * col * max(0.75, f32(ceil(y) % 2));
}
