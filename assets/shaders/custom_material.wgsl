// #import bevy_pbr::forward_io::VertexOutput
// // we can import items from shader modules in the assets folder with a quoted path
// #import "shaders/custom_material_import.wgsl"::COLOR_MULTIPLIER

#import bevy_pbr::mesh_functions::{get_world_from_local, mesh_position_local_to_clip}

// Vertex shader
struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
};

// struct VertexOutput {
//     @builtin(position) clip_position: vec4<f32>,
//     @location(0) world_position: vec3<f32>,
//     @location(1) world_normal: vec3<f32>,
// };


struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec3<f32>,
    @location(1) world_normal: vec3<f32>,
};

@vertex
fn vertex(input: Vertex) -> VertexOutput {
    var output: VertexOutput;

    output.clip_position = mesh_position_local_to_clip(
        get_world_from_local(input.instance_index),
        vec4<f32>(input.position, 1.0),
    );

    output.world_position = input.position;
    output.world_normal = normalize(input.normal);

    return output;
}

struct FragmentInput {
    @location(0) world_position: vec3<f32>,
    @location(1) world_normal: vec3<f32>,
}

// struct Light {
//     position: vec3<f32>,
//     color: vec3<f32>,
//     intensity: f32,
// };
// @group(1) @binding(0) var<uniform> light: Light;

@group(2) @binding(0) var<uniform> color: vec3<f32>;

@fragment
fn fragment(input: FragmentInput) -> @location(0) vec4<f32> {
    const light_pos: vec3<f32> = vec3(-2.0, 2.5, 5.0);
    const light_int: f32 = 0.6;
    // const light_col: vec3<f32> = vec3(1.0, 1.0, 1.0);

    let normal = normalize(input.world_normal);
    let light_dir = normalize(light_pos - input.world_position);
    let diffuse = max(dot(normal, light_dir), 0.0) * light_int;
    let diff_col = color * diffuse;

    return vec4(diff_col, 1.0);
}

// @fragment
// fn fragment_main(input: FragmentInput) -> @location(0) vec4<f32> {
//     let normal = normalize(input.world_normal);
//     let light_dir = normalize(light.position - input.world_position);
//     let diffuse = max(dot(normal, light_dir), 0.0) * light.intensity;
//     let diffuse_color = light.color * diffuse;
//     return vec4<f32>(diffuse_color, 1.0); // Output final color
// }

// struct FragmentInput {
//     // @location(0) blend_color: vec4<f32>,
// };

