use raylib::prelude::*;

mod vec3;
mod ray;
mod material;
mod geometry;
mod light;
mod camera;
mod scene;
mod raytracer;

use vec3::Vec3;
use material::Material;
use geometry::{Cube, Plane};
use light::Light;
use camera::Camera;
use scene::Scene;
use raytracer::Raytracer;

const SCREEN_WIDTH: i32 = 800;
const SCREEN_HEIGHT: i32 = 600;

fn main() {

    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Clean Raytracer")
        .build();

    let raytracer = Raytracer::new(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32);
    let scene = create_scene();
    
    // Camera positioned to view cube from above and at an angle to see 2 sides
    let camera = Camera::new(
        Vec3::new(3.0, 4.0, 2.0),      // Position: higher and to the side
        Vec3::new(0.0, -0.5, -3.0),    // Look at: the cube center
        Vec3::up(),
        45.0,
        SCREEN_WIDTH as f32 / SCREEN_HEIGHT as f32,
    );

    let image_buffer = raytracer.render(&scene, &camera);
    println!("Rendering complete!");

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let pixel = image_buffer[y as usize][x as usize];
                d.draw_pixel(x, y, pixel);
            }
        }

    }
}

fn create_scene() -> Scene {
    let mut scene = Scene::new();
    
    // Change background to white
    scene.set_background_color(Vec3::new(1.0, 1.0, 1.0));
    
    // Add light gray floor
    let floor_material = Material::new()
        .with_color(Vec3::new(0.7, 0.7, 0.7))  // Light gray
        .with_roughness(0.9);
    
    scene.add_plane(Plane::new(
        Vec3::new(0.0, -2.0, 0.0),    // Position
        Vec3::up(),                   // Normal pointing up
        floor_material,
    ));
    
    let checkerboard_material = Material::new()
        .with_checkerboard(
            1.0,
            Vec3::new(1.0, 0.0, 1.0), // Magenta
            Vec3::new(0.0, 0.0, 0.0)  // Black
        )
        .with_roughness(0.3)
        .with_specular(0.5)
        .with_reflectivity(0.2);
    
    // Only one cube now
    scene.add_cube(Cube::new(
        Vec3::new(0.0, -0.5, -3.0),
        Vec3::new(1.5, 1.5, 1.5),
        checkerboard_material,
    ));

    scene.add_light(Light::new(
        Vec3::new(-3.0, 5.0, 2.0),
        Vec3::new(1.0, 1.0, 0.9),
        1.0,
    ));
    
    scene
}
