use crate::material::Material;
use crate::ray_intersect::{Intersect, RayIntersect};
use raylib::prelude::Vector3;

pub struct Cube {
    pub center: Vector3,
    pub size: f32, // Half the side length (distance from center to face)
    pub material: Material,
}

impl RayIntersect for Cube {
    fn ray_intersect(&self, ray_origin: &Vector3, ray_direction: &Vector3) -> Intersect {
        // Calculate the min and max bounds of the cube
        let min = Vector3::new(
            self.center.x - self.size,
            self.center.y - self.size,
            self.center.z - self.size,
        );
        let max = Vector3::new(
            self.center.x + self.size,
            self.center.y + self.size,
            self.center.z + self.size,
        );

        // Calculate intersection parameters for each axis
        let inv_dir = Vector3::new(
            1.0 / ray_direction.x,
            1.0 / ray_direction.y,
            1.0 / ray_direction.z,
        );

        let t1 = (min.x - ray_origin.x) * inv_dir.x;
        let t2 = (max.x - ray_origin.x) * inv_dir.x;
        let t3 = (min.y - ray_origin.y) * inv_dir.y;
        let t4 = (max.y - ray_origin.y) * inv_dir.y;
        let t5 = (min.z - ray_origin.z) * inv_dir.z;
        let t6 = (max.z - ray_origin.z) * inv_dir.z;

        let tmin_x = t1.min(t2);
        let tmax_x = t1.max(t2);
        let tmin_y = t3.min(t4);
        let tmax_y = t3.max(t4);
        let tmin_z = t5.min(t6);
        let tmax_z = t5.max(t6);

        let tmin = tmin_x.max(tmin_y).max(tmin_z);
        let tmax = tmax_x.min(tmax_y).min(tmax_z);

        // Check if ray intersects the cube
        if tmax < 0.0 || tmin > tmax {
            return Intersect::empty();
        }

        // Use the closest intersection point
        let t = if tmin > 0.0 { tmin } else { tmax };
        
        if t <= 0.0 {
            return Intersect::empty();
        }

        let point = *ray_origin + *ray_direction * t;
        
        // Calculate normal based on which face was hit
        let normal = self.calculate_normal(&point);
        
        // Calculate UV coordinates for texture mapping
        let (u, v) = self.calculate_uv(&point, &normal);

        Intersect::new(point, normal, t, self.material.clone(), u, v)
    }
}

impl Cube {
    fn calculate_normal(&self, point: &Vector3) -> Vector3 {
        // Calculate which face the point is closest to
        let relative = *point - self.center;
        
        let abs_x = relative.x.abs();
        let abs_y = relative.y.abs();
        let abs_z = relative.z.abs();
        
        // Find the axis with the largest component (closest to a face)
        if abs_x >= abs_y && abs_x >= abs_z {
            // Hit the X face
            if relative.x > 0.0 {
                Vector3::new(1.0, 0.0, 0.0) // Right face
            } else {
                Vector3::new(-1.0, 0.0, 0.0) // Left face
            }
        } else if abs_y >= abs_x && abs_y >= abs_z {
            // Hit the Y face
            if relative.y > 0.0 {
                Vector3::new(0.0, 1.0, 0.0) // Top face
            } else {
                Vector3::new(0.0, -1.0, 0.0) // Bottom face
            }
        } else {
            // Hit the Z face
            if relative.z > 0.0 {
                Vector3::new(0.0, 0.0, 1.0) // Front face
            } else {
                Vector3::new(0.0, 0.0, -1.0) // Back face
            }
        }
    }

    fn calculate_uv(&self, point: &Vector3, normal: &Vector3) -> (f32, f32) {
        // Calculate UV coordinates based on which face was hit
        let relative = *point - self.center;
        
        // Normalize to [-1, 1] range based on cube size
        let normalized = Vector3::new(
            relative.x / self.size,
            relative.y / self.size,
            relative.z / self.size,
        );

        // Calculate UV coordinates based on the face normal
        let (u, v) = match (normal.x.abs() > 0.5, normal.y.abs() > 0.5, normal.z.abs() > 0.5) {
            (true, false, false) => {
                // X face (left/right)
                ((normalized.z + 1.0) * 0.5, (normalized.y + 1.0) * 0.5)
            }
            (false, true, false) => {
                // Y face (top/bottom)
                ((normalized.x + 1.0) * 0.5, (normalized.z + 1.0) * 0.5)
            }
            (false, false, true) => {
                // Z face (front/back)
                ((normalized.x + 1.0) * 0.5, (normalized.y + 1.0) * 0.5)
            }
            _ => (0.0, 0.0), // Fallback, shouldn't happen
        };

        (u, v)
    }
}