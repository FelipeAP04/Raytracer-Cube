// Example scene configurations for the Clean Raytracer
// Copy these examples into the create_scene() function in main.rs

// Example 1: Multiple cubes
/*
// Add a second cube
scene.add_cube(Cube::new(
    Vec3::new(2.0, -0.5, -4.0),
    Vec3::new(1.0, 1.0, 1.0),
    checkerboard_material.clone(),
));
*/

// Example 2: Different materials
/*
let red_material = Material::new()
    .with_color(Vec3::new(1.0, 0.2, 0.2))  // Red
    .with_specular(0.8)
    .with_reflectivity(0.5);
*/

// Example 3: Alternative camera angles
/*
// Side view
let camera = Camera::new(
    Vec3::new(5.0, 0.0, -3.0),
    Vec3::new(0.0, -0.5, -3.0),
    Vec3::up(),
    45.0,
    aspect_ratio,
);

// Front view
let camera = Camera::new(
    Vec3::new(0.0, 0.0, 2.0),
    Vec3::new(0.0, -0.5, -3.0),
    Vec3::up(),
    45.0,
    aspect_ratio,
);
*/

// Example 4: Different lighting setups
/*
// Add multiple lights
scene.add_light(Light::new(
    Vec3::new(3.0, 3.0, 0.0),   // Position
    Vec3::new(0.8, 0.8, 1.0),   // Blue tint
    0.7,                        // Intensity
));

scene.add_light(Light::new(
    Vec3::new(-2.0, 4.0, -2.0), // Position
    Vec3::new(1.0, 0.8, 0.6),   // Warm tint
    0.5,                        // Intensity
));
*/

// Example 5: Colored floor
/*
let colored_floor = Material::new()
    .with_color(Vec3::new(0.3, 0.5, 0.3))  // Green tint
    .with_roughness(0.8);
*/
