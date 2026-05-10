//! Mesh extended

use macroquad::prelude::*;
use macroquad::ui::Vertex;

/// https://github.com/raysan5/raylib/blob/master/src/rmodels.c#L2772
#[rustfmt::skip]
pub fn gen_cube(width: f32, height: f32, length: f32) -> Mesh {
    static TEXCOORDS: [f32; 48] = [
        0.0, 0.0,
        1.0, 0.0,
        1.0, 1.0,
        0.0, 1.0,
        1.0, 0.0,
        1.0, 1.0,
        0.0, 1.0,
        0.0, 0.0,
        0.0, 1.0,
        0.0, 0.0,
        1.0, 0.0,
        1.0, 1.0,
        1.0, 1.0,
        0.0, 1.0,
        0.0, 0.0,
        1.0, 0.0,
        1.0, 0.0,
        1.0, 1.0,
        0.0, 1.0,
        0.0, 0.0,
        0.0, 0.0,
        1.0, 0.0,
        1.0, 1.0,
        0.0, 1.0
    ];

     static NORMALS: [f32; 72] = [
        0.0, 0.0, 1.0,
        0.0, 0.0, 1.0,
        0.0, 0.0, 1.0,
        0.0, 0.0, 1.0,
        0.0, 0.0,-1.0,
        0.0, 0.0,-1.0,
        0.0, 0.0,-1.0,
        0.0, 0.0,-1.0,
        0.0, 1.0, 0.0,
        0.0, 1.0, 0.0,
        0.0, 1.0, 0.0,
        0.0, 1.0, 0.0,
        0.0,-1.0, 0.0,
        0.0,-1.0, 0.0,
        0.0,-1.0, 0.0,
        0.0,-1.0, 0.0,
        1.0, 0.0, 0.0,
        1.0, 0.0, 0.0,
        1.0, 0.0, 0.0,
        1.0, 0.0, 0.0,
        -1.0, 0.0, 0.0,
        -1.0, 0.0, 0.0,
        -1.0, 0.0, 0.0,
        -1.0, 0.0, 0.0
    ];

    const fn v(x: f32, y: f32, z: f32) -> Vertex {
        Vertex {
            position: vec3(x, y, z),
            uv: vec2(0.0, 0.0),
            color: [255; 4],
            normal: Vec4::ZERO
        }
    }

    let vertices = vec![
        v(-width/2., -height/2., length/2.),
        v(width/2., -height/2., length/2.),
        v(width/2., height/2., length/2.),
        v(-width/2., height/2., length/2.),
        v(-width/2., -height/2., -length/2.),
        v(-width/2., height/2., -length/2.),
        v(width/2., height/2., -length/2.),
        v(width/2., -height/2., -length/2.),
        v(-width/2., height/2., -length/2.),
        v(-width/2., height/2., length/2.),
        v(width/2., height/2., length/2.),
        v(width/2., height/2., -length/2.),
        v(-width/2., -height/2., -length/2.),
        v(width/2., -height/2., -length/2.),
        v(width/2., -height/2., length/2.),
        v(-width/2., -height/2., length/2.),
        v(width/2., -height/2., -length/2.),
        v(width/2., height/2., -length/2.),
        v(width/2., height/2., length/2.),
        v(width/2., -height/2., length/2.),
        v(-width/2., -height/2., -length/2.),
        v(-width/2., -height/2., length/2.),
        v(-width/2., height/2., length/2.),
        v(-width/2., height/2., -length/2.),
    ]
    .iter_mut().enumerate().map(|(i, v)| {
        v.uv = vec2(TEXCOORDS[i * 2], TEXCOORDS[i * 2 + 1]);
        v.normal = vec4(NORMALS[i * 3], NORMALS[i * 3 + 1], NORMALS[i * 3 + 2], 0.0);
        *v
    }).collect();


    let mut indices = vec![0u16; 36];

    let mut k = 0;

    for i in (0..36).step_by(6) {
        indices[i] = 4 * k;
        indices[i + 1] = 4*k + 1;
        indices[i + 2] = 4*k + 2;
        indices[i + 3] = 4*k;
        indices[i + 4] = 4*k + 2;
        indices[i + 5] = 4*k + 3;

        k += 1;
    }

    Mesh {
        indices,
        vertices,
        texture: None,
    }
}
