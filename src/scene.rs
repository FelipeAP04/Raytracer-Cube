// Scene management for raytracing

use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::geometry::{Cube, Plane, HittableList, HitRecord, Hittable};
use crate::light::Light;

const EPSILON: f32 = 0.001;

#[derive(Debug)]
pub struct Scene {
    pub objects: HittableList,
    pub lights: Vec<Light>,
    pub background_color: Vec3,
    pub ambient_light: Vec3,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            objects: HittableList::new(),
            lights: Vec::new(),
            background_color: Vec3::new(0.1, 0.1, 0.2),
            ambient_light: Vec3::new(0.1, 0.1, 0.1),
        }
    }

    pub fn set_background_color(&mut self, color: Vec3) {
        self.background_color = color;
    }

    pub fn add_cube(&mut self, cube: Cube) {
        self.objects.add(cube);
    }

    pub fn add_plane(&mut self, plane: Plane) {
        self.objects.add(plane);
    }

    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }

    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.objects.hit(ray, t_min, t_max)
    }

    pub fn get_background_color(&self, _ray: &Ray) -> Vec3 {
        self.background_color
    }

    pub fn is_in_shadow(&self, from: Vec3, to: Vec3) -> bool {
        let direction = to - from;
        let distance = direction.length();
        let ray = Ray::new(from, direction.normalize());

        if let Some(_hit) = self.hit(&ray, EPSILON, distance - EPSILON) {
            true
        } else {
            false
        }
    }

    pub fn get_lights_affecting_point(&self, point: Vec3) -> Vec<(&Light, f32)> {
        let mut affecting_lights = Vec::new();

        for light in &self.lights {
            let shadow_factor = if self.is_in_shadow(point, light.position) {
                0.3
            } else {
                1.0
            };

            affecting_lights.push((light, shadow_factor));
        }

        affecting_lights
    }
}
