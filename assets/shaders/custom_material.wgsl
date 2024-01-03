#import bevy_pbr::mesh_vertex_output MeshVertexOutput
#import bevy_pbr::mesh_view_bindings globals

struct CustomMaterial {
    color: vec4<f32>,
};

@group(1) @binding(0)
var<uniform> material: CustomMaterial;
@group(1) @binding(1)
var base_color_texture: texture_2d<f32>;
@group(1) @binding(2)
var base_color_sampler: sampler;

@fragment
fn fragment(
    mesh: MeshVertexOutput,
) -> @location(0) vec4<f32> {
    let speed = 2.0;
    let repeated_uv = fract(mesh.uv);
    let scrolled_uv = repeated_uv+ vec2<f32>(cos(globals.time*speed), cos(globals.time*speed));
    return material.color * textureSample(base_color_texture, base_color_sampler, mesh.uv);
}
