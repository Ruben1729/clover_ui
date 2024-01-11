struct VertexOutput {
    @location(0) out_color: vec4<f32>,
    @builtin(position) position: vec4<f32>,
};

struct Uniforms {
    screenSize: vec2<f32>
}

@group(0) @binding(0) var <uniform> uniforms: Uniforms;

@vertex
fn vs_main(@location(0) a_position: vec4<f32>, @location(1) a_color: vec4<f32>) -> VertexOutput {
    var result: VertexOutput;
    result.out_color = a_color;
    result.position = vec4<f32>(
                       (a_position.x / uniforms.screenSize.x - 0.5) * 2.0,
                       (0.5 - a_position.y / uniforms.screenSize.y) * 2.0,
                       a_position.z / 10000.0,
                       a_position.w
                     );

    // Output the transformed vertex position
    return result;
}

@fragment
fn fs_main(vertex: VertexOutput) -> @location(0) vec4<f32> {
    // Output the interpolated color
    return vertex.out_color;
}
