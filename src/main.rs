use raylib::prelude::*;
use std::f32::consts::PI;

mod framebuffer;
mod ray_intersect;
mod cube;
mod camera;
mod light;
mod material;
// mod texture; // Commented out for performance

use framebuffer::Framebuffer;
use ray_intersect::{Intersect, RayIntersect};
use cube::Cube;
use camera::Camera;
use light::Light;
use material::{Material, vector3_to_color};
// use texture::Texture; // Commented out for performance

const ORIGIN_BIAS: f32 = 1e-4;

fn procedural_sky(dir: Vector3) -> Vector3 {
    let d = dir.normalized();
    let t = (d.y + 1.0) * 0.5; // map y [-1,1] → [0,1]

    let green = Vector3::new(0.5, 0.2, 0.2);
    let white = Vector3::new(0.1, 0.05, 0.05);
    let blue = Vector3::new(0.5, 0.2, 0.2);

    if t < 0.54 {
        // Bottom → fade green to white
        let k = t / 0.55;
        green * (1.0 - k) + white * k
    } else if t < 0.55 {
        // Around horizon → mostly white
        white
    } else if t < 0.8 {
        // Fade white to blue
        let k = (t - 0.55) / (0.25);
        white * (1.0 - k) + blue * k
    } else {
        // Upper sky → solid blue
        blue
    }
}

fn offset_origin(intersect: &Intersect, direction: &Vector3) -> Vector3 {
    let offset = intersect.normal * ORIGIN_BIAS;
    if direction.dot(intersect.normal) < 0.0 {
        intersect.point - offset
    } else {
        intersect.point + offset
    }
}

fn reflect(incident: &Vector3, normal: &Vector3) -> Vector3 {
    *incident - *normal * 2.0 * incident.dot(*normal)
}

fn refract(incident: &Vector3, normal: &Vector3, refractive_index: f32) -> Option<Vector3> {
    // Implementation of Snell's Law for refraction.
    // It calculates the direction of a ray as it passes from one medium to another.

    // `cosi` is the cosine of the angle between the incident ray and the normal.
    // We clamp it to the [-1, 1] range to avoid floating point errors.
    let mut cosi = incident.dot(*normal).max(-1.0).min(1.0);

    // `etai` is the refractive index of the medium the ray is currently in.
    // `etat` is the refractive index of the medium the ray is entering.
    // `n` is the normal vector, which may be flipped depending on the ray's direction.
    let mut etai = 1.0; // Assume we are in Air (or vacuum) initially
    let mut etat = refractive_index;
    let mut n = *normal;

    if cosi > 0.0 {
        // The ray is inside the medium (e.g., glass) and going out into the air.
        // We need to swap the refractive indices.
        std::mem::swap(&mut etai, &mut etat);
        // We also flip the normal so it points away from the medium.
        n = -n;
    } else {
        // The ray is outside the medium and going in.
        // We need a positive cosine for the calculation, so we negate it.
        cosi = -cosi;
    }

    // `eta` is the ratio of the refractive indices (n1 / n2).
    let eta = etai / etat;
    // `k` is a term derived from Snell's law that helps determine if total internal reflection occurs.
    let k = 1.0 - eta * eta * (1.0 - cosi * cosi);

    if k < 0.0 {
        // If k is negative, it means total internal reflection has occurred.
        // There is no refracted ray, so we return None.
        None
    } else {
        // If k is non-negative, we can calculate the direction of the refracted ray.
        Some(*incident * eta + n * (eta * cosi - k.sqrt()))
    }
}

fn cast_shadow(
    intersect: &Intersect,
    light: &Light,
    objects: &[Cube],
) -> f32 {
    let light_dir = (light.position - intersect.point).normalized();
    let light_distance = (light.position - intersect.point).length();

    let shadow_ray_origin = offset_origin(intersect, &light_dir);

    for object in objects {
        let shadow_intersect = object.ray_intersect(&shadow_ray_origin, &light_dir);
        if shadow_intersect.is_intersecting && shadow_intersect.distance < light_distance {
            return 1.0; // Hit something, full shadow
        }
    }

    0.0 // No shadow
}

pub fn cast_ray(
    ray_origin: &Vector3,
    ray_direction: &Vector3,
    objects: &[Cube],
    lights: &[Light],
    depth: u32,
) -> Vector3 {
    if depth > 3 {
        return procedural_sky(*ray_direction);
        // return SKYBOX_COLOR;
    }

    let mut intersect = Intersect::empty();
    let mut zbuffer = f32::INFINITY;

    for object in objects {
        let i = object.ray_intersect(ray_origin, ray_direction);
        if i.is_intersecting && i.distance < zbuffer {
            zbuffer = i.distance;
            intersect = i;
        }
    }

    if !intersect.is_intersecting {
        return procedural_sky(*ray_direction);
        // return SKYBOX_COLOR;
    }

    let view_dir = (*ray_origin - intersect.point).normalized();
    let material_color = intersect.material.get_diffuse_color(intersect.u, intersect.v);
    let albedo = intersect.material.albedo;
    
    // Accumulate lighting from all light sources
    let mut total_diffuse = Vector3::zero();
    let mut total_specular = Vector3::zero();
    
    for light in lights {
        let light_dir = (light.position - intersect.point).normalized();
        let reflect_dir = reflect(&-light_dir, &intersect.normal).normalized();
        
        let shadow_intensity = cast_shadow(&intersect, light, objects);
        let light_intensity = light.intensity * (1.0 - shadow_intensity);
        
        let diffuse_intensity = intersect.normal.dot(light_dir).max(0.0) * light_intensity;
        let light_color_v3 = Vector3::new(light.color.r as f32 / 255.0, light.color.g as f32 / 255.0, light.color.b as f32 / 255.0);
        
        total_diffuse = total_diffuse + material_color * diffuse_intensity * light_color_v3;
        
        let specular_intensity = view_dir.dot(reflect_dir).max(0.0).powf(intersect.material.specular) * light_intensity;
        total_specular = total_specular + light_color_v3 * specular_intensity;
    }
    
    let phong_color = total_diffuse * albedo[0] + total_specular * albedo[1];

    // Reflections
    let reflectivity = intersect.material.albedo[2];
    let reflect_color = if reflectivity > 0.0 {
        let reflect_dir = reflect(ray_direction, &intersect.normal).normalized();
        let reflect_origin = offset_origin(&intersect, &reflect_dir);
        cast_ray(&reflect_origin, &reflect_dir, objects, lights, depth + 1)
    } else {
        Vector3::zero()
    };

    // Refractions
    let transparency = intersect.material.albedo[3];
    let refract_color = if transparency > 0.0 {
        // Calculate the refracted ray direction. This can fail (return None) in case of total internal reflection.
        if let Some(refract_dir) = refract(ray_direction, &intersect.normal, intersect.material.refractive_index) {
            // If refraction is possible, cast a new ray.
            let refract_origin = offset_origin(&intersect, &refract_dir);
            cast_ray(&refract_origin, &refract_dir, objects, lights, depth + 1)
        } else {
            // Total internal reflection occurred. In this case, the light is perfectly reflected.
            // We cast a reflection ray instead of a refraction ray.
            let reflect_dir = reflect(ray_direction, &intersect.normal).normalized();
            let reflect_origin = offset_origin(&intersect, &reflect_dir);
            cast_ray(&reflect_origin, &reflect_dir, objects, lights, depth + 1)
        }
    } else {
        // If the material is not transparent, the refracted color is black.
        Vector3::zero()
    };

    // Combine the Phong color with the reflected and refracted colors using the material's albedo values.
    phong_color * (1.0 - reflectivity - transparency) + reflect_color * reflectivity + refract_color * transparency
}

pub fn render(framebuffer: &mut Framebuffer, objects: &[Cube], camera: &Camera, lights: &[Light]) {
    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;
    let aspect_ratio = width / height;
    let fov = PI / 3.0;
    let perspective_scale = (fov * 0.5).tan();

    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            let screen_x = (2.0 * x as f32) / width - 1.0;
            let screen_y = -(2.0 * y as f32) / height + 1.0;

            let screen_x = screen_x * aspect_ratio * perspective_scale;
            let screen_y = screen_y * perspective_scale;

            let ray_direction = Vector3::new(screen_x, screen_y, -1.0).normalized();
            
            let rotated_direction = camera.basis_change(&ray_direction);

            let pixel_color_v3 = cast_ray(&camera.eye, &rotated_direction, objects, lights, 0);
            let pixel_color = vector3_to_color(pixel_color_v3);

            framebuffer.set_current_color(pixel_color);
            framebuffer.set_pixel(x, y);
        }
    }
}

fn main() {
    let window_width = 1300;
    let window_height = 900;
 
    let (mut window, thread) = raylib::init()
        .size(window_width, window_height)
        .title("Raytracer Example")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    let mut framebuffer = Framebuffer::new(window_width as u32, window_height as u32);

    // Stone material for the pedestal (gray stone-like color with matte finish)
    let stone = Material::new(
        Vector3::new(0.5, 0.5, 0.5), // Gray stone color
        3.0,  // Much lower specular exponent for matte finish
        [0.95, 0.05, 0.0, 0.0], // Almost entirely diffuse, minimal specular
        0.0,
    );

    // Lantern material - glowing and semi-transparent to let light through
    let lantern = Material::new(
        Vector3::new(1.0, 0.8, 0.4), // Warm yellow/orange glow
        10.0,
        [0.0, 0.9, 0.0, 0.9], // Some transparency to let light through
        1.2,
    );

    // Steel material - highly reflective metallic surface (fixed transparency)
    let steel = Material::new(
        Vector3::new(0.7, 0.7, 0.8), // Slightly bluish metallic color
        100.0, // High specular exponent for sharp reflections
        [0.4, 0.4, 0.0, 0.0], // More diffuse/specular, less reflective to avoid transparency
        0.0,
    );

    // Wood material - matte, natural surface
    let wood = Material::new(
        Vector3::new(0.6, 0.4, 0.2), // Brown wood color
        3.0,  // Very low specular exponent for matte finish
        [0.98, 0.02, 0.0, 0.0], // Almost entirely diffuse, very minimal specular
        0.0,
    );

    // Iron material - more reflective and lighter than steel
    let iron = Material::new(
        Vector3::new(0.85, 0.85, 0.9), // Lighter, brighter metallic color
        120.0, // Even higher specular exponent for sharper reflections
        [0.2, 0.3, 0.5, 0.0], // More reflective than steel (50% vs 30%)
        0.0,
    );

    let objects = [
        // Pedestal - 5x5 base (25 cubes) with 0.5 size
        // Bottom row (y = -1.0) - 5x5 grid
        Cube::new_uniform(Vector3::new(-1.0, -1.0, -1.0), 0.25, stone.clone()),
        Cube::new_uniform(Vector3::new(-0.5, -1.0, -1.0), 0.25, stone.clone()),
        Cube::new_uniform(Vector3::new( 0.0, -1.0, -1.0), 0.25, stone.clone()),
        Cube::new_uniform(Vector3::new( 0.5, -1.0, -1.0), 0.25, stone.clone()),
        Cube::new_uniform(Vector3::new( 1.0, -1.0, -1.0), 0.25, stone.clone()),
        
        Cube::new_uniform(Vector3::new(-1.0, -1.0, -0.5), 0.25, stone.clone()),
        Cube::new_uniform(Vector3::new(-0.5, -1.0, -0.5), 0.25, stone.clone()),
        Cube::new_uniform(Vector3::new( 0.0, -1.0, -0.5), 0.25, stone.clone()),
        Cube::new_uniform(Vector3::new( 0.5, -1.0, -0.5), 0.25, stone.clone()),
        Cube::new_uniform(Vector3::new( 1.0, -1.0, -0.5), 0.25, stone.clone()),
        
        Cube::new_uniform(Vector3::new(-1.0, -1.0,  0.0), 0.25, stone.clone()),
        Cube::new_uniform(Vector3::new(-0.5, -1.0,  0.0), 0.25, stone.clone()),
        Cube::new_uniform(Vector3::new( 0.0, -1.0,  0.0), 0.25, stone.clone()),
        Cube::new_uniform(Vector3::new( 0.5, -1.0,  0.0), 0.25, stone.clone()),
        Cube::new_uniform(Vector3::new( 1.0, -1.0,  0.0), 0.25, stone.clone()),
        
        Cube::new_uniform(Vector3::new(-1.0, -1.0,  0.5), 0.25, stone.clone()),
        Cube::new_uniform(Vector3::new(-0.5, -1.0,  0.5), 0.25, stone.clone()),
        Cube::new_uniform(Vector3::new( 0.0, -1.0,  0.5), 0.25, stone.clone()),
        Cube::new_uniform(Vector3::new( 0.5, -1.0,  0.5), 0.25, stone.clone()),
        Cube::new_uniform(Vector3::new( 1.0, -1.0,  0.5), 0.25, stone.clone()),
        
        Cube::new_uniform(Vector3::new(-1.0, -1.0,  1.0), 0.25, stone.clone()),
        Cube::new_uniform(Vector3::new(-0.5, -1.0,  1.0), 0.25, stone.clone()),
        Cube::new_uniform(Vector3::new( 0.0, -1.0,  1.0), 0.25, stone.clone()),
        Cube::new_uniform(Vector3::new( 0.5, -1.0,  1.0), 0.25, stone.clone()),
        Cube::new_uniform(Vector3::new( 1.0, -1.0,  1.0), 0.25, stone.clone()),
        
        // Upper level (y = -0.5) - 3x3 grid
        Cube::new_uniform(Vector3::new(-0.5, -0.5, -0.5), 0.25, stone.clone()),
        Cube::new_uniform(Vector3::new( 0.0, -0.5, -0.5), 0.25, stone.clone()),
        Cube::new_uniform(Vector3::new( 0.5, -0.5, -0.5), 0.25, stone.clone()),
        
        Cube::new_uniform(Vector3::new(-0.5, -0.5,  0.0), 0.25, stone.clone()),
        Cube::new_uniform(Vector3::new( 0.0, -0.5,  0.0), 0.25, stone.clone()),
        Cube::new_uniform(Vector3::new( 0.5, -0.5,  0.0), 0.25, stone.clone()),
        
        Cube::new_uniform(Vector3::new(-0.5, -0.5,  0.5), 0.25, stone.clone()),
        Cube::new_uniform(Vector3::new( 0.0, -0.5,  0.5), 0.25, stone.clone()),
        Cube::new_uniform(Vector3::new( 0.5, -0.5,  0.5), 0.25, stone.clone()),

        // Floating stone cubes
        Cube::new_uniform(Vector3::new( 1.0, 1.5,  1.0), 0.25, stone.clone()),
        Cube::new_uniform(Vector3::new(-1.0, 1.5,  1.0), 0.25, stone.clone()),
        Cube::new_uniform(Vector3::new( 1.0, 1.5, -1.0), 0.25, stone.clone()),
        Cube::new_uniform(Vector3::new(-1.0, 1.5, -1.0), 0.25, stone.clone()),

        // Lanterns
        Cube::new_box(Vector3::new( 1.0, 1.17,  1.0), 0.15, 0.2, 0.15, lantern.clone()),
        Cube::new_box(Vector3::new(-1.0, 1.17,  1.0), 0.15, 0.2, 0.15, lantern.clone()),
        Cube::new_box(Vector3::new( 1.0, 1.17, -1.0), 0.15, 0.2, 0.15, lantern.clone()),
        Cube::new_box(Vector3::new(-1.0, 1.17, -1.0), 0.15, 0.2, 0.15, lantern.clone()),
        
        // Anvil
        Cube::new_box(Vector3::new(0.0,-0.20, 0.0), 0.20, 0.10, 0.14, steel.clone()),
        Cube::new_box(Vector3::new(0.0,-0.10, 0.0), 0.15, 0.12, 0.10, steel.clone()),
        Cube::new_box(Vector3::new(0.0, 0.0, 0.0), 0.25, 0.10, 0.18, steel.clone()),

        // Sword on anvil
        Cube::new_box(Vector3::new(0.0, 0.1, 0.0), 0.04, 0.13, 0.015, iron.clone()),

    ];

    let mut camera = Camera::new(
        Vector3::new(1.0, 1.0, 5.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
    );
    let rotation_speed = PI / 30.0;

    // Main scene light (reduced intensity for softer lighting)
    let main_light = Light::new(
        Vector3::new(1.0, 0.5, 5.0),
        Color::new(255, 255, 255, 255),
        0.4,  // Reduced from 1.0 to 0.6 for less intense lighting
    );
    
    // Lantern lights
    let lantern_lights = [
        Light::new(
            Vector3::new(1.0, 1.0, 1.0),
            Color::new(255, 200, 100, 255),
            0.5,
        ),
        Light::new(
            Vector3::new(-1.0, 1.0, 1.0),
            Color::new(255, 200, 100, 255),
            0.5,
        ),
        Light::new(
            Vector3::new(1.0, 1.0, -1.0),
            Color::new(255, 200, 100, 255),
            0.5,
        ),
        Light::new(
            Vector3::new(-1.0, 1.0, -1.0),
            Color::new(255, 200, 100, 255),
            0.5,
        ),
    ];
    
    // Combine all lights
    let mut all_lights = vec![main_light];
    all_lights.extend_from_slice(&lantern_lights);

    let move_speed = 0.9; // Movement speed for forward/backward
    
    while !window.window_should_close() {
        // Orbital controls (arrow keys)
        if window.is_key_down(KeyboardKey::KEY_LEFT) {
            camera.orbit(rotation_speed, 0.0);  // Left: rotate yaw only (horizontal)
        }
        if window.is_key_down(KeyboardKey::KEY_RIGHT) {
            camera.orbit(-rotation_speed, 0.0); // Right: rotate yaw only (horizontal)
        }
        if window.is_key_down(KeyboardKey::KEY_UP) {
            camera.orbit(0.0, -rotation_speed);  // Up: rotate pitch only (vertical)
        }
        if window.is_key_down(KeyboardKey::KEY_DOWN) {
            camera.orbit(0.0, rotation_speed);   // Down: rotate pitch only (vertical)
        }
        
        // Forward/backward movement (W/S keys)
        if window.is_key_down(KeyboardKey::KEY_W) {
            // Move forward towards the center point
            let direction = (camera.center - camera.eye).normalized();
            camera.eye = camera.eye + direction * move_speed;
            camera.update_basis_vectors();
        }
        if window.is_key_down(KeyboardKey::KEY_S) {
            // Move backward away from the center point
            let direction = (camera.center - camera.eye).normalized();
            camera.eye = camera.eye - direction * move_speed;
            camera.update_basis_vectors();
        }

        framebuffer.clear();
        render(&mut framebuffer, &objects, &camera, &all_lights);
        framebuffer.swap_buffers(&mut window, &thread);
    }
}