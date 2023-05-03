#import bevy_pbr::mesh_types
// The time since startup data is in the globals binding which is part of the mesh_view_bindings import
#import bevy_pbr::mesh_view_bindings

struct VertexOutput{
    @builtin(position) clip_position: vec4<f32>,
};

struct FragmentInput {
    #import bevy_pbr::mesh_vertex_output
}

struct Focus{
    x: f32,
    y: f32,
    zoom: f32
}

@group(1) @binding(0) 
var<uniform> focus: Focus;

fn hsv_to_rgb(h: f32, s: f32, v: f32) -> vec4<f32>{
    let c = v * s;
    let x = c * (1.0 - abs(((h/60.0) % 2.0) - 1.0));
    let m = v - c;

    
    if (h < 60.0){
        return vec4<f32>(c, x, 0.0, 1.0);
    }else if (h < 120.0){
        return vec4<f32>(x, c, 0.0, 1.0);
    }else if (h < 180.0){
        return vec4<f32>(0.0, c, x, 1.0);
    }else if (h < 240.0){
        return vec4<f32>(0.0, x, c, 1.0);
    }else if (h < 300.0){
        return vec4<f32>(x, 0.0, c, 1.0);
    }else {
        return vec4<f32>(c, 0.0, x, 1.0);
    }
}

fn mul_complex(c1: vec2<f32>, c2: vec2<f32>) -> vec2<f32>{
    let x = (c1[0]*c1[0]) - (c1[1]*c1[1]);
    let y = (c1[0]*c1[1]) + (c1[1]*c1[0]);

    return vec2(x, y);
}

@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {
    let zoom = pow(focus.zoom, 3.0);
    let x = ((in.uv.x - 0.5) * ((16.0* 32.0)/ zoom)) - focus.x;
    let y = ((in.uv.y - 0.5) * ((9.0 * 32.0) / zoom)) - focus.y;

    var z = vec2(0.0,0.0);
    let c = vec2(x, y);

    var n = 0.0;
    let max = 25000.0;
    
    let escape_radius = 2.5;
    while (length(z) <= escape_radius){
        z = mul_complex(z,z) + c;
        n = n + (1.0/max);

        if (n > 1.0){
            return vec4<f32>(0.0, 0.0, 0.0, 1.0);
        }
    }
    
    return hsv_to_rgb(n*360.0, 0.5, 0.5);
}