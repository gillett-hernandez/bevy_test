// #import bevy_pbr::mesh_vertex_output
// #import bevy_sprite::mesh2d_vertex_output
#import bevy_sprite::mesh2d_bindings
#import bevy_sprite::mesh2d_view_bindings
// #import bevy_sprite::mesh2d_types
#import bevy_sprite::mesh2d_functions

struct CustomMaterial {
    color: vec4<f32>,
    circle_size: f32,
};

@group(1) @binding(0)
var<uniform> material: CustomMaterial;
@group(1) @binding(1)
var<uniform> circle_size: f32;
// @group(1) @binding(2)
// var color_sampler: sampler;

@fragment
fn fragment(
    #import bevy_sprite::mesh2d_vertex_output
) -> @location(0) vec4<f32> {
    let offset_uv = uv - vec2<f32>(0.5, 0.5);

    // if (offset_uv.x * offset_uv.x + offset_uv.y * offset_uv.y) > 0.1 {
    if (offset_uv.x * offset_uv.x + offset_uv.y * offset_uv.y) > (circle_size * circle_size) {
        return vec4<f32>(1.0,1.0,1.0,0.2);
    } else {
        return vec4<f32>(0.0, 0.0, 0.0, 0.0);
    }
}
