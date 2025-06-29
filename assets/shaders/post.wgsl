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

// STRIPES
// @fragment
// fn fs_main(vs: VertexOutput) -> @location(0) vec4<f32> {
//     let col = textureSample(input_image, input_sampler, vs.uv);
//     let scale_y = f32(globals.viewport_size.y / globals.render_size.y);

//     let y = vs.uv.y * f32(globals.viewport_size.y);
//     let stripe_y = min(1., 0.75 * floor(y % scale_y));
//     return stripe_y * col;
//     // return vec4(col.r, col.g * stripe_y, col.b, col.a);
// }

// DOTS
@fragment
fn fs_main(vs: VertexOutput) -> @location(0) vec4<f32> {
     
    // CHROMATIC

    // var col = vec4(1.);
    // let pixel_size = 1. / f32(globals.viewport_size.x);
    // let offset = vec2(pixel_size);
    // col.r = textureSample(input_image, input_sampler, vs.uv - offset).r;
    // col.g = textureSample(input_image, input_sampler, vs.uv).g;
    // col.b = textureSample(input_image, input_sampler, vs.uv + offset).b;

    // DOTS

    let col = textureSample(input_image, input_sampler, vs.uv);
    let scale_x = f32(globals.viewport_size.x / globals.render_size.x);
    let scale_y = f32(globals.viewport_size.y / globals.render_size.y);

    let y = vs.uv.y * f32(globals.viewport_size.y);
    let x = vs.uv.x * f32(globals.viewport_size.x);
    let stripe_y = floor(y % scale_y);
    let stripe_x = floor(x % scale_x);

    let luma = dot(col, vec4(.299, .587, .114, .0));
    let v = min(1., 0.5 * luma + max(stripe_x, stripe_y));
    return vec4(col.r * v, col.g * v, col.b * v, col.a);
}

// GRID
// const STRIPE_MIN = 0.8;
// @fragment
// fn fs_main(vs: VertexOutput) -> @location(0) vec4<f32> {
//     let col = textureSample(input_image, input_sampler, vs.uv);
//     let scale_x = f32(globals.viewport_size.x / globals.render_size.x);
//     let scale_y = f32(globals.viewport_size.y / globals.render_size.y);

//     let y = vs.uv.y * f32(globals.viewport_size.y);
//     let stripe_y = min(1., STRIPE_MIN + floor(2 * y / scale_y) % 2);
//     let x = vs.uv.x * f32(globals.viewport_size.x);
//     let stripe_x = min(1., STRIPE_MIN + floor(2 * x / scale_x) % 2);
//     return vec4(col.r * stripe_x, col.g * stripe_y, col.b, col.a);
// }

// MOSAIC
// const MASK = array(
//     .9, .8, .85,
//     .82, .93, .9,
//     .95, .87, .92,
// );
// const MASK_SIZE: u32 = 3;
 
// @fragment
// fn fs_main(vs: VertexOutput) -> @location(0) vec4<f32> {
//     let col = textureSample(input_image, input_sampler, vs.uv);
//     let scale_x = f32(globals.viewport_size.x / globals.render_size.x);
//     let scale_y = f32(globals.viewport_size.y / globals.render_size.y);

//     let y = vs.uv.y * f32(globals.viewport_size.y);
//     let x = vs.uv.x * f32(globals.viewport_size.x);

//     let stripe_y = u32(floor(y / scale_y) % f32(MASK_SIZE));
//     let stripe_x = u32(floor(x / scale_x) % f32(MASK_SIZE));
     
//     return col * min(1., MASK[stripe_y * MASK_SIZE + stripe_x]);
//     // return vec4(col.r * stripe_x, col.g * stripe_y, col.b, col.a);
// }

// CRT MASK
// const MASK_SIZE = 12.;
// const MASK_BORDER = 0.8;

// @fragment
// fn fs_main(vs: VertexOutput) -> @location(0) vec4<f32> {
//     let col = textureSample(input_image, input_sampler, vs.uv);
//     // let scale_x = f32(globals.viewport_size.x / globals.render_size.x);
//     // let scale_y = f32(globals.viewport_size.y / globals.render_size.y);

//     let y = vs.uv.y * f32(globals.viewport_size.y);
//     let x = vs.uv.x * f32(globals.viewport_size.x);
//     let pixel = vec2(x, y);
//     let coord = pixel / MASK_SIZE;
//     let subcoord = coord * vec2(3., 1.);

//     let cell_offset = vec2(0., fract(floor(coord.x) * 0.5));
//     let idx = floor(subcoord.x) % 3.;
//     var mask_color = 3. * vec4(f32(idx == 0.), f32(idx == 1.), f32(idx == 2.), 1.);

//     let cell_uv = fract(subcoord + cell_offset) * 2. - 1.;
//     let border = 1. - cell_uv * cell_uv * MASK_BORDER;
//     mask_color *= border.x * border.y;
//     mask_color.a = 1.;
     
//     return col * mask_color;
//     // return col;
// }

 
// CHROMATIC ABBERATION
// const OFFSET = vec2(0.002, 0.002);
 
// @fragment
// fn fs_main(vs: VertexOutput) -> @location(0) vec4<f32> {
//     let val = distance(vs.uv, vec2(0.5, 0.5));
//     var col = vec4(1.);
//     let offset = val * OFFSET;
//     col.r = textureSample(input_image, input_sampler, vs.uv - offset).r;
//     col.g = textureSample(input_image, input_sampler, vs.uv).g;
//     col.b = textureSample(input_image, input_sampler, vs.uv + offset).b;
//     return col;
// }
