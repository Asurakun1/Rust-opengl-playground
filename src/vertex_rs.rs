pub fn program(display: &glium::Display) -> glium::Program {
    glium::Program::from_source(display, vertex_shader(), fragment_shader(), None).unwrap()
}

fn vertex_shader<'a>() -> &'a str {
    r#"
    #version 150


    in vec3 position;
    in vec3 normal;
    in vec2 tex_coords;


    out vec3 v_normal;
    out vec3 v_position;
    out vec2 v_tex_coords;


    uniform mat4 perspective;
    uniform mat4 view;
    uniform mat4 model;


    void main() {
        v_tex_coords = tex_coords;
        mat4 modelview = view * model;
        v_normal = transpose(inverse(mat3(modelview))) * normal;
        gl_Position = perspective * modelview * vec4(position, 1.0);
        v_position = gl_Position.xyz / gl_Position.w;
    }
    "#
}

fn fragment_shader<'a>() -> &'a str {
    r#"
    #version 140

    in vec3 v_normal;
    in vec2 v_tex_coords;
    in vec3 v_position;

    out vec4 color;
    uniform vec3 u_light;
    uniform sampler2D diffuse_tex;
    uniform sampler2D normal_tex;

    mat3 cotangent_frame(vec3 normal, vec3 pos, vec2 uv) {
        vec3 dp1 = dFdx(pos);
        vec3 dp2 = dFdy(pos);
        vec2 duv1 = dFdx(uv);
        vec2 duv2 = dFdy(uv);
    
        vec3 dp2perp = cross(dp2, normal);
        vec3 dp1perp = cross(normal, dp1);
        vec3 T = dp2perp * duv1.x + dp1perp * duv2.x;
        vec3 B = dp2perp * duv1.y + dp1perp * duv2.y;
    
        float invmax = inversesqrt(max(dot(T, T), dot(B, B)));
        return mat3(T * invmax, B * invmax, normal);
    }
    
    void main() {
        vec3 normal_map = texture(normal_tex, v_tex_coords).rgb;

        mat3 tbn = cotangent_frame(v_normal, -v_position, v_tex_coords);
        vec3 real_normal = normalize(tbn * -(normal_map * 2.0 - 1.0));

        vec3 diffuse_color = texture(diffuse_tex, v_tex_coords).rgb;
        vec3 ambient_color = diffuse_color * 0.1;

        float brightness = max(dot(normalize(real_normal), normalize(u_light)), 0.0);
        vec3 dark_color = vec3(0.6, 0.0, 0.0);
        vec3 regular_color = vec3(1.0, 0.0, 0.0);
        color = vec4(mix(real_normal, ambient_color, brightness), 1.0);
    }
    "#
}

pub fn get_indices() -> glium::index::NoIndices {
    glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip)
}
