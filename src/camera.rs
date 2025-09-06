// Camera system for raytracing

use crate::vec3::Vec3;
use crate::ray::Ray;

fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * std::f32::consts::PI / 180.0
}

#[derive(Debug, Clone)]
pub struct Camera {
    pub position: Vec3,
    pub target: Vec3,
    pub up: Vec3,
    pub fov: f32,
    pub aspect_ratio: f32,
    pub forward: Vec3,
    pub right: Vec3,
    pub camera_up: Vec3,
    pub focal_length: f32,
}

impl Camera {
    pub fn new(position: Vec3, target: Vec3, up: Vec3, fov: f32, aspect_ratio: f32) -> Self {
        let mut camera = Camera {
            position,
            target,
            up,
            fov,
            aspect_ratio,
            forward: Vec3::zero(),
            right: Vec3::zero(),
            camera_up: Vec3::zero(),
            focal_length: 1.0,
        };

        camera.update_camera_vectors();
        camera
    }

    fn update_camera_vectors(&mut self) {
        self.forward = (self.target - self.position).normalize();
        self.right = self.forward.cross(&self.up).normalize();
        self.camera_up = self.right.cross(&self.forward).normalize();
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let ndc_x = (u * 2.0) - 1.0;
        let ndc_y = (v * 2.0) - 1.0;

        let half_width = (degrees_to_radians(self.fov) * 0.5).tan() * self.aspect_ratio;
        let half_height = (degrees_to_radians(self.fov) * 0.5).tan();

        let target_point = self.position + self.forward * self.focal_length
                          + self.right * (ndc_x * half_width)
                          + self.camera_up * (ndc_y * half_height);

        let direction = (target_point - self.position).normalize();

        Ray::new(self.position, direction)
    }
}
