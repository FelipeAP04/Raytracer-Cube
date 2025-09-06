// Geometry primitives for raytracing

use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::material::Material;

const EPSILON: f32 = 0.001;

#[derive(Debug, Clone)]
pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub material: Material,
}

impl HitRecord {
    pub fn new(point: Vec3, normal: Vec3, t: f32, ray: &Ray, material: Material) -> Self {
        let front_face = ray.direction.dot(&normal) < 0.0;
        let normal = if front_face { normal } else { -normal };

        HitRecord {
            point,
            normal,
            t,
            material,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

#[derive(Debug, Clone)]
pub struct Cube {
    pub min: Vec3,
    pub max: Vec3,
    pub material: Material,
}

impl Cube {
    pub fn new(center: Vec3, size: Vec3, material: Material) -> Self {
        let half_size = size * 0.5;
        Cube {
            min: center - half_size,
            max: center + half_size,
            material,
        }
    }
}

impl Hittable for Cube {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut t_min = t_min;
        let mut t_max = t_max;
        let mut hit_normal = Vec3::new(0.0, 0.0, 0.0);

        // Check each pair of planes (x, y, z)
        for axis in 0..3 {
            let ray_dir = match axis {
                0 => ray.direction.x,
                1 => ray.direction.y,
                _ => ray.direction.z,
            };

            let ray_origin = match axis {
                0 => ray.origin.x,
                1 => ray.origin.y,
                _ => ray.origin.z,
            };

            let min_val = match axis {
                0 => self.min.x,
                1 => self.min.y,
                _ => self.min.z,
            };

            let max_val = match axis {
                0 => self.max.x,
                1 => self.max.y,
                _ => self.max.z,
            };

            let inv_dir = 1.0 / ray_dir;
            let mut t0 = (min_val - ray_origin) * inv_dir;
            let mut t1 = (max_val - ray_origin) * inv_dir;

            if inv_dir < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }

            if t0 > t_min {
                t_min = t0;
                hit_normal = Vec3::new(0.0, 0.0, 0.0);
                match axis {
                    0 => hit_normal.x = if inv_dir < 0.0 { 1.0 } else { -1.0 },
                    1 => hit_normal.y = if inv_dir < 0.0 { 1.0 } else { -1.0 },
                    _ => hit_normal.z = if inv_dir < 0.0 { 1.0 } else { -1.0 },
                }
            }

            if t1 < t_max {
                t_max = t1;
            }

            if t_max < t_min {
                return None;
            }
        }

        let t = if t_min > EPSILON { t_min } else { t_max };

        if t < EPSILON {
            return None;
        }

        let point = ray.at(t);

        Some(HitRecord::new(point, hit_normal, t, ray, self.material.clone()))
    }
}

#[derive(Debug, Clone)]
pub struct Plane {
    pub point: Vec3,
    pub normal: Vec3,
    pub material: Material,
}

impl Plane {
    pub fn new(point: Vec3, normal: Vec3, material: Material) -> Self {
        Plane {
            point,
            normal: normal.normalize(),
            material,
        }
    }
}

impl Hittable for Plane {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let denom = self.normal.dot(&ray.direction);
        
        if denom.abs() < EPSILON {
            return None; // Ray is parallel to plane
        }
        
        let t = (self.point - ray.origin).dot(&self.normal) / denom;
        
        if t < t_min || t > t_max {
            return None;
        }
        
        let point = ray.at(t);
        Some(HitRecord::new(point, self.normal, t, ray, self.material.clone()))
    }
}

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl std::fmt::Debug for HittableList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HittableList")
            .field("objects", &format!("{} objects", self.objects.len()))
            .finish()
    }
}

impl HittableList {
    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn add<T: Hittable + 'static>(&mut self, object: T) {
        self.objects.push(Box::new(object));
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_hit: Option<HitRecord> = None;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if let Some(hit_record) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = hit_record.t;
                closest_hit = Some(hit_record);
            }
        }

        closest_hit
    }
}
