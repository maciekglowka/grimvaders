struct GlobalsUniform {
    time: f32,
    _padding_0: f32,
    render_size: vec2<u32>,
    viewport_size: vec2<u32>,
    _padding_1: f32,
    _padding_2: f32,
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

const STRIPE_MIN = 0.9;

@fragment
fn fs_main(vs: VertexOutput) -> @location(0) vec4<f32> {
    let col = textureSample(input_image, input_sampler, vs.uv);
    let scale_x = f32(globals.viewport_size.x / globals.render_size.x);
    let scale_y = f32(globals.viewport_size.y / globals.render_size.y);

    let y = vs.uv.y * f32(globals.viewport_size.y);
    let stripe_y = min(1., STRIPE_MIN + floor(y / scale_y) % 2);
    let x = vs.uv.x * f32(globals.viewport_size.x);
    let stripe_x = min(1., STRIPE_MIN + floor(x / scale_x) % 2);
    return vec4(col.r * stripe_x, col.g * stripe_y, col.b, col.a);
}
