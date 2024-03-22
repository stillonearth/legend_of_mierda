#import bevy_pbr::{
    utils, 
}
#import bevy_core_pipeline::fullscreen_vertex_shader FullscreenVertexOutput


@group(0) @binding(0)
var screen_texture: texture_2d<f32>;
@group(0) @binding(1)
var texture_sampler: sampler;
struct PostProcessSettings {
    height: f32,
    width: f32,
    noise: f32,
}
@group(0) @binding(2)
var<uniform> shader_settings : PostProcessSettings;

const SCALE: f32 = 2.0;

fn downsample(in: vec2<f32>) -> vec2<f32> {
    return in;
    // return vec2<f32>(floor(in.x * shader_settings.width / SCALE) / shader_settings.width * SCALE, floor(in.y * shader_settings.height / SCALE) / shader_settings.height * SCALE);
}

fn permute_four(x: vec4<f32>) -> vec4<f32> { return ((x * 34. + 1.) * x) % vec4<f32>(289.); }
fn fade_two(t: vec2<f32>) -> vec2<f32> { return t * t * t * (t * (t * 6. - 15.) + 10.); }

fn perlin_noise_2d(P: vec2<f32>) -> f32 {
  var Pi: vec4<f32> = floor(P.xyxy) + vec4<f32>(0., 0., 1., 1.);
  let Pf = fract(P.xyxy) - vec4<f32>(0., 0., 1., 1.);
  Pi = Pi % vec4<f32>(289.); // To avoid truncation effects in permutation
  let ix = Pi.xzxz;
  let iy = Pi.yyww;
  let fx = Pf.xzxz;
  let fy = Pf.yyww;
  let i = permute_four(permute_four(ix) + iy);
  var gx: vec4<f32> = 2. * fract(i * 0.0243902439) - 1.; // 1/41 = 0.024...
  let gy = abs(gx) - 0.5;
  let tx = floor(gx + 0.5);
  gx = gx - tx;
  var g00: vec2<f32> = vec2<f32>(gx.x, gy.x);
  var g10: vec2<f32> = vec2<f32>(gx.y, gy.y);
  var g01: vec2<f32> = vec2<f32>(gx.z, gy.z);
  var g11: vec2<f32> = vec2<f32>(gx.w, gy.w);
  let norm = 1.79284291400159 - 0.85373472095314 *
      vec4<f32>(dot(g00, g00), dot(g01, g01), dot(g10, g10), dot(g11, g11));
  g00 = g00 * norm.x;
  g01 = g01 * norm.y;
  g10 = g10 * norm.z;
  g11 = g11 * norm.w;
  let n00 = dot(g00, vec2<f32>(fx.x, fy.x));
  let n10 = dot(g10, vec2<f32>(fx.y, fy.y));
  let n01 = dot(g01, vec2<f32>(fx.z, fy.z));
  let n11 = dot(g11, vec2<f32>(fx.w, fy.w));
  let fade_xy = fade_two(Pf.xy);
  let n_x = mix(vec2<f32>(n00, n01), vec2<f32>(n10, n11), vec2<f32>(fade_xy.x));
  let n_xy = mix(n_x.x, n_x.y, fade_xy.y);
  return 2.3 * n_xy;
}

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    // downsample
    var uv = downsample(in.uv);
    var input = vec2<f32>((uv.x + shader_settings.noise) * 600.0 , (uv.y + shader_settings.noise) * 600.0);
    var noise = perlin_noise_2d(input);
    var alpha = (noise + 1.0) / 2.0;

    // color quantize
    var color = textureSample(screen_texture, texture_sampler, uv);
    color = floor(color * 32.0) / 32.0 ;

    return vec4<f32>(color.rgb, 1.0) * vec4<f32>(alpha, alpha, alpha, alpha);
}