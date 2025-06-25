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


const GLOW = 0.5;
const KERNEL_DIM = 2;
const THRESH_MIN = 0.05;
const THRESH_MAX = 0.125;

@fragment
fn fs_main(vs: VertexOutput) -> @location(0) vec4<f32> {
    let col = textureSample(input_image, input_sampler, vs.uv);

    let dx = 1.0 / f32(globals.viewport_size.x);
    let dy = 1.0 / f32(globals.viewport_size.y);

    var n = vec4(0.);

    for (var x=-KERNEL_DIM; x<=KERNEL_DIM; x++) {
        for (var y=-KERNEL_DIM; y<=KERNEL_DIM; y++) {
            let g = textureSample(input_image, input_sampler, vs.uv + vec2(f32(x) * dx, f32(y) * dy));
            n += max(vec4(0.), g - col);
        }
    }

    n /= f32((2 * KERNEL_DIM + 1) * (2 * KERNEL_DIM + 1));

    let glow = smoothstep(vec4(THRESH_MIN), vec4(THRESH_MAX), n) * n;
    return col + GLOW * glow;
}
