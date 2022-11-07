struct Output { 
    @builtin(position) position : vec4<f32>,
    @location(0) v_color : vec4<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> Output { 
    var pos : array<vec2<f32>, 9> = array<vec2<f32>, 9>( 
        vec2<f32>(-0.63, 0.80), 
        vec2<f32>(-0.65, 0.20), 
        vec2<f32>(-0.20, 0.60), 
        vec2<f32>(-0.37, -0.07), 
        vec2<f32>( 0.05, 0.18), 
        vec2<f32>(-0.13, -0.40), 
        vec2<f32>( 0.30, -0.13), 
        vec2<f32>( 0.13, -0.64), 
        vec2<f32>( 0.70, -0.30) 
    ); 
    var color : array<vec3<f32>, 9> = array<vec3<f32>, 9>( 
        vec3<f32>(1.0, 0.0, 0.0), 
        vec3<f32>(0.0, 1.0, 0.0), 
        vec3<f32>(0.0, 0.0, 1.0), 
        vec3<f32>(1.0, 0.0, 0.0), 
        vec3<f32>(0.0, 1.0, 0.0), 
        vec3<f32>(0.0, 0.0, 1.0), 
        vec3<f32>(1.0, 0.0, 0.0), 
        vec3<f32>(0.0, 1.0, 0.0), 
        vec3<f32>(0.0, 0.0, 1.0), 
    ); 
    var output: Output;
    output.position = vec4<f32>(pos[in_vertex_index], 0.0, 1.0); 
    output.v_color = vec4<f32>(color[in_vertex_index], 1.0);
    return output; 
}

@fragment
fn fs_main(@location(0) v_color: vec4<f32>) -> @location(0) vec4<f32> { 
    return v_color;
}
