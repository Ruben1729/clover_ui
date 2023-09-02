struct VertexOutput {
    @location(0) out_color: vec4<f32>,
    @builtin(position) position: vec4<f32>,
};


struct Uniforms {
    screenSize: vec2<f32>
}

@group(0) @binding(0) var <uniform> uniforms: Uniforms;

@vertex
fn vs_main(@location(0) a_position: vec2<f32>, @location(1) a_color: u32) -> VertexOutput {
    // Unpack the u32 color into 4 components
    let r: f32 = f32((a_color >> 24u) & 0xFFu) / 255.0;
    let g: f32 = f32((a_color >> 16u) & 0xFFu) / 255.0;
    let b: f32 = f32((a_color >> 8u) & 0xFFu) / 255.0;
    let a: f32 = f32(a_color & 0xFFu) / 255.0;

    var position = vec2<f32>(
                       (a_position.x / uniforms.screenSize.x - 0.5) * 2.0,
                       (0.5 - a_position.y / uniforms.screenSize.y) * 2.0
                     );

    var result: VertexOutput;
    result.out_color = vec4<f32>(r, g, b, a);
    result.position = vec4<f32>(position,0.0,1.0);

    // Output the transformed vertex position
    return result;
}

@fragment
fn fs_main(vertex: VertexOutput) -> @location(0) vec4<f32> {
    // Output the interpolated color
    return vertex.out_color;
}
