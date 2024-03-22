#import bevy_pbr::utils
#import bevy_core_pipeline::fullscreen_vertex_shader FullscreenVertexOutput
#import bevy_shader_utils::perlin_noise_3d

@group(0) @binding(0)
var screen_texture: texture_2d<f32>;
@group(0) @binding(1)
var texture_sampler: sampler;
struct PostProcessSettings {
    height: f32,
    width: f32,
}
@group(0) @binding(2)
var<uniform> window_resolution : PostProcessSettings;

const SCALE: f32 = 2.0;

fn downsample(in: vec2<f32>) -> vec2<f32> {
    return in;
    // return vec2<f32>(floor(in.x * window_resolution.width / SCALE) / window_resolution.width * SCALE, floor(in.y * window_resolution.height / SCALE) / window_resolution.height * SCALE);
}

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    // downsample
    var uv = downsample(in.uv);

    // color quantize
    var color = textureSample(screen_texture, texture_sampler, uv);
    color = floor(color * 32.0) / 32.0;

    return vec4<f32>(color.rgb, 1.0);
}