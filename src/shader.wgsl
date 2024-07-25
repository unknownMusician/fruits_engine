@group(0) @binding(0)
var<uniform> matrix_world_to_clip: mat4x4<f32>;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(2) color: vec4<f32>,
};

struct InstanceData {
    @location(5) local_to_world_c0: vec4<f32>,
    @location(6) local_to_world_c1: vec4<f32>,
    @location(7) local_to_world_c2: vec4<f32>,
    @location(8) local_to_world_c3: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@vertex
fn vs_main(vertex: VertexInput, instance: InstanceData) -> VertexOutput {
    var out: VertexOutput;

    var matrix_local_to_world = get_matrix_local_to_world(instance);

    var world_position = matrix_local_to_world * vec4<f32>(vertex.position, 1.0);
    
    out.clip_position = matrix_world_to_clip * world_position;
    out.color = vertex.color.xyz;
    
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}

fn get_matrix_local_to_world(instance: InstanceData) -> mat4x4<f32> {
    return mat4x4<f32>(
        instance.local_to_world_c0,
        instance.local_to_world_c1,
        instance.local_to_world_c2,
        instance.local_to_world_c3
    );
}