// Material system for raytracing

use crate::vec3::Vec3;

#[derive(Debug, Clone)]
pub enum TextureType {
    SolidColor,
    Checkerboard { scale: f32, color1: Vec3, color2: Vec3 },
}

#[derive(Debug, Clone)]
pub struct Material {
    pub color: Vec3,
    pub texture: TextureType,
    pub specular: f32,
    pub roughness: f32,
    pub reflectivity: f32,
    pub emission: Vec3,
}

impl Material {
    pub fn new() -> Self {
        Material {
            color: Vec3::new(0.7, 0.7, 0.7),
            texture: TextureType::SolidColor,
            specular: 0.1,
            roughness: 0.8,
            reflectivity: 0.0,
            emission: Vec3::zero(),
        }
    }

    pub fn with_checkerboard(mut self, scale: f32, color1: Vec3, color2: Vec3) -> Self {
        self.texture = TextureType::Checkerboard { scale, color1, color2 };
        self
    }

    pub fn with_color(mut self, color: Vec3) -> Self {
        self.color = color;
        self
    }

    pub fn with_specular(mut self, specular: f32) -> Self {
        self.specular = specular.clamp(0.0, 1.0);
        self
    }

    pub fn with_roughness(mut self, roughness: f32) -> Self {
        self.roughness = roughness.clamp(0.0, 1.0);
        self
    }

    pub fn with_reflectivity(mut self, reflectivity: f32) -> Self {
        self.reflectivity = reflectivity.clamp(0.0, 1.0);
        self
    }

    pub fn emitted(&self) -> Vec3 {
        self.emission
    }

    pub fn get_color_at_point(&self, point: &Vec3) -> Vec3 {
        match &self.texture {
            TextureType::SolidColor => self.color,
            TextureType::Checkerboard { scale, color1, color2 } => {
                let x_check = (point.x * scale).floor() as i32;
                let y_check = (point.y * scale).floor() as i32;
                let z_check = (point.z * scale).floor() as i32;

                if (x_check + y_check + z_check) % 2 == 0 {
                    *color1
                } else {
                    *color2
                }
            }
        }
    }
}
