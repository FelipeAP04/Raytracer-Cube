// Lighting system for raytracing

use crate::vec3::Vec3;

#[derive(Debug, Clone)]
pub struct Light {
    pub position: Vec3,
    pub color: Vec3,
    pub intensity: f32,
}

impl Light {
    pub fn new(position: Vec3, color: Vec3, intensity: f32) -> Self {
        Light {
            position,
            color,
            intensity,
        }
    }

    pub fn get_direction_from(&self, point: Vec3) -> Vec3 {
        (self.position - point).normalize()
    }

    pub fn get_effective_color(&self, point: Vec3) -> Vec3 {
        let distance = (self.position - point).length();
        let attenuation = 1.0 / (1.0 + 0.1 * distance + 0.01 * distance * distance);
        self.color * self.intensity * attenuation
    }
}
