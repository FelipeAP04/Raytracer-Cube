// Raytracing engine

use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::scene::Scene;
use crate::camera::Camera;
use crate::geometry::HitRecord;
use raylib::prelude::*;

pub struct Raytracer {
    pub width: u32,
    pub height: u32,
    pub max_depth: i32,
}

impl Raytracer {
    pub fn new(width: u32, height: u32) -> Self {
        Raytracer {
            width,
            height,
            max_depth: 5,
        }
    }

    pub fn render(&self, scene: &Scene, camera: &Camera) -> Vec<Vec<Color>> {
        let mut image = vec![vec![Color::BLACK; self.width as usize]; self.height as usize];

        println!("Rendering {}x{} pixels", self.width, self.height);

        for y in 0..self.height {
            for x in 0..self.width {
                let u = x as f32 / (self.width - 1) as f32;
                let v = (self.height - 1 - y) as f32 / (self.height - 1) as f32;

                let ray = camera.get_ray(u, v);
                let color = self.ray_color(&ray, scene, self.max_depth);

                image[y as usize][x as usize] = vec3_to_color(color);
            }

            if y % 100 == 0 {
                println!("Line {} of {}", y, self.height);
            }
        }

        println!("Rendering complete!");
        image
    }

    fn ray_color(&self, ray: &Ray, scene: &Scene, depth: i32) -> Vec3 {
        if depth <= 0 {
            return Vec3::zero();
        }

        if let Some(hit_record) = scene.hit(ray, 0.001, f32::INFINITY) {
            self.calculate_lighting(&hit_record, ray, scene, depth)
        } else {
            scene.get_background_color(ray)
        }
    }

    fn calculate_lighting(&self, hit: &HitRecord, incident_ray: &Ray, scene: &Scene, depth: i32) -> Vec3 {
        let mut color = Vec3::zero();

        let material_color = hit.material.get_color_at_point(&hit.point);

        color += hit.material.emitted();
        color += scene.ambient_light * material_color;

        for (light, shadow_factor) in scene.get_lights_affecting_point(hit.point) {
            let light_dir = light.get_direction_from(hit.point);
            let light_color = light.get_effective_color(hit.point);

            let diffuse_strength = hit.normal.dot(&light_dir).max(0.0);
            let diffuse = material_color * light_color * diffuse_strength * shadow_factor;
            color += diffuse;

            if hit.material.specular > 0.0 && diffuse_strength > 0.0 {
                let view_dir = (-incident_ray.direction).normalize();
                let reflect_dir = (-light_dir).reflect(&hit.normal);

                let spec_strength = view_dir.dot(&reflect_dir).max(0.0)
                    .powf((1.0 - hit.material.roughness) * 128.0);

                let specular = light_color * hit.material.specular * spec_strength * shadow_factor;
                color += specular;
            }
        }

        if hit.material.reflectivity > 0.0 && depth > 1 {
            let reflected = incident_ray.direction.reflect(&hit.normal);
            let reflection_ray = Ray::new(hit.point + hit.normal * 0.001, reflected);
            let reflection_color = self.ray_color(&reflection_ray, scene, depth - 1);
            color += reflection_color * hit.material.reflectivity;
        }

        color.clamp(0.0, 1.0)
    }
}

fn vec3_to_color(color: Vec3) -> Color {
    Color::new(
        (color.x.clamp(0.0, 1.0) * 255.0) as u8,
        (color.y.clamp(0.0, 1.0) * 255.0) as u8,
        (color.z.clamp(0.0, 1.0) * 255.0) as u8,
        255,
    )
}
