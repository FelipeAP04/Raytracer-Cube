use raylib::prelude::{Color, Vector3};
// use crate::texture::Texture; // Commented out for performance

#[derive(Debug, Clone)]
pub struct Material {
    pub diffuse: Vector3,
    pub albedo: [f32; 4],
    pub specular: f32,
    pub refractive_index: f32,
    // pub texture: Option<Texture>, // Commented out for performance
}

impl Material {
    pub fn get_diffuse_color(&self, _u: f32, _v: f32) -> Vector3 {
        // Texture support commented out for performance
        // match &self.texture {
        //     Some(texture) => texture.sample(u, v),
        //     None => self.diffuse,
        // }
        self.diffuse
    }
}

impl Material {
    pub fn new(diffuse: Vector3, specular: f32, albedo: [f32; 4], refractive_index: f32) -> Self {
        Material {
            diffuse,
            albedo,
            specular,
            refractive_index,
            // texture: None, // Commented out for performance
        }
    }

    // Commented out for performance - texture support disabled
    // pub fn new_with_texture(diffuse: Vector3, specular: f32, albedo: [f32; 4], refractive_index: f32, texture: Texture) -> Self {
    //     Material {
    //         diffuse,
    //         albedo,
    //         specular,
    //         refractive_index,
    //         texture: Some(texture),
    //     }
    // }

    pub fn black() -> Self {
        Material {
            diffuse: Vector3::zero(),
            albedo: [0.0, 0.0, 0.0, 0.0],
            specular: 0.0,
            refractive_index: 0.0,
            // texture: None, // Commented out for performance
        }
    }
}

pub fn vector3_to_color(v: Vector3) -> Color {
    Color::new(
        (v.x * 255.0).min(255.0) as u8,
        (v.y * 255.0).min(255.0) as u8,
        (v.z * 255.0).min(255.0) as u8,
        255,
    )
}