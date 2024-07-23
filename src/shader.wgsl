@group(0) @binding(0)
var<uniform> matrix_world_to_clip: mat4x4<f32>;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(2) color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    
    out.color = model.color.xyz;
    out.clip_position = matrix_world_to_clip * vec4<f32>(model.position, 1.0);

    //out.clip_position.z %= 1.0;
    
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}