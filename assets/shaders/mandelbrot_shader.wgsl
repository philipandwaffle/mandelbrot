#import bevy_pbr::mesh_types
// The time since startup data is in the globals binding which is part of the mesh_view_bindings import
#import bevy_pbr::mesh_view_bindings

struct VertexOutput{
    @builtin(position) clip_position: vec4<f32>,
};

struct FragmentInput {
    #import bevy_pbr::mesh_vertex_output
}

const pi: f32 = 3.14159265358979323846264338327950288;

struct Focus{
    x: f32,
    y: f32,
    z_x: f32,
    z_y: f32,
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
    let a = c1[0];
    let b = c1[1];
    let c = c2[0];
    let d = c2[1];

    let x = a*c - b*d;
    let y = a*d + b*c;

    return vec2(x, y);
}

fn pow_complex(z: vec2<f32>, n: f32) -> vec2<f32>{
    let a = z[0];
    let b = z[1];

    let r = pow(sqrt(a*a + b*b), n);

    var theta = 0.0;
    if (a != 0.0){
        theta = atan(b/a);
    }
    
    if (theta>(pi/4.0) && theta<(pi/2.0)){
        theta = theta + pi;
    }else if (theta>(pi/2.0) && theta<(3.0*pi)/4.0){
        theta = theta - pi;
    }

    let x = r * cos(n*theta);
    let y = r * sin(n*theta);
    return vec2(x,y);
}

fn simple_pow_complex(z: vec2<f32>, n: f32) -> vec2<f32>{
    var res = z;
    var count = n;
    while (count > 1.0){
        res = mul_complex(res,z);
        count -= 1.0;
    }
    return res;
}

fn div_complex(c1: vec2<f32>, c2: vec2<f32>) -> vec2<f32>{
    let a = c1[0];
    let b = c1[1];
    let c = c2[0];
    let d = c2[1];

    let x = (a*c + b*d) / (c*c + d*d);
    let y = (b*c + a*d) / (c*c + d*d);

    return vec2(x, y);
}

@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {
    let zoom = pow(focus.zoom, 3.0);
    let x = ((in.uv.x - 0.5) * ((16.0* 32.0) / zoom * 0.2)) - focus.x;
    let y = ((in.uv.y - 0.5) * ((9.0 * 32.0) / zoom * 0.2)) - focus.y;

    var z = vec2(focus.z_x, focus.z_x);
    let c = vec2(x,y);

    var n = 0.0;
    let max = 2500.0;
    
    let escape_radius = 20.0;
    let z_0 = z;
    while (length(z) <= escape_radius){
        // z = mul_complex(z, z) + c;
        // z = pow_complex(z, 3.3) + c;
        // z = mul_complex(z,mul_complex(z,z)) + c;
        // z = mul_complex(mul_complex(mul_complex(z,z),z),z) + c;
        // z = mul_complex(z,z) / (z+c);
        // z = div_complex(mul_complex(mul_complex(z,z),z), (3.0*mul_complex(z,z)));
        // z = div_complex(simple_pow_complex(z,15.0), (3.0*mul_complex(z,z))) + c;
        z = pow_complex(div_complex(mul_complex(z,z)+ c, 2.0*z), 0.8);

        n = n + (1.0/max);

        if (n > 1.0){
            return vec4<f32>(0.0, 0.0, 0.0, 1.0);
        }
    }
    
    return hsv_to_rgb(pow(n, 0.6)*360.0, 0.5, 0.5);
}