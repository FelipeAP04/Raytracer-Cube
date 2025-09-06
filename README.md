# Clean Raytracer

A simple, educational raytracer implementation in Rust using Raylib for rendering.

## Features

- **3D Ray Tracing**: Basic ray-object intersection and lighting calculations
- **Checkerboard Textures**: Procedural texture generation
- **Phong Lighting Model**: Diffuse and specular lighting with shadows
- **Geometric Primitives**: Support for cubes and infinite planes
- **Real-time Display**: Uses Raylib for window management and pixel rendering
- **Clean Architecture**: Flat module structure for easy understanding

## Demo Scene

The current scene showcases:
- A checkerboard-textured cube (magenta and black pattern)
- Light gray floor plane
- White background for clean aesthetics
- Single point light with shadow casting
- Camera positioned for optimal 3D view (top-down angle showing 2 sides)

## Project Structure

```
src/
├── main.rs          # Entry point and scene setup
├── vec3.rs          # 3D vector mathematics
├── ray.rs           # Ray structure for raytracing
├── material.rs      # Material system with texture support
├── geometry.rs      # Geometric primitives (Cube, Plane)
├── light.rs         # Lighting system
├── camera.rs        # Camera system with perspective projection
├── scene.rs         # Scene management
└── raytracer.rs     # Core raytracing engine
```

## Prerequisites

- Rust (latest stable version)
- Raylib dependencies (automatically handled by cargo)

### Linux Dependencies
```bash
# Ubuntu/Debian
sudo apt install libasound2-dev mesa-common-dev libx11-dev libxrandr-dev libxi-dev xorg-dev libgl1-mesa-dev libglu1-mesa-dev

# Arch Linux
sudo pacman -S alsa-lib mesa libx11 libxrandr libxi libxcursor libxinerama
```

## Building and Running

```bash
# Clone the repository
git clone <repository-url>
cd raytracer_clean

# Build the project
cargo build --release

# Run the raytracer
cargo run --release
```

## Controls

- **ESC**: Exit the application
- The scene renders once and displays the result

## Technical Details

### Raytracing Pipeline
1. **Ray Generation**: Camera generates rays for each pixel
2. **Intersection Testing**: Rays are tested against scene geometry
3. **Lighting Calculation**: Phong model with ambient, diffuse, and specular components
4. **Shadow Calculation**: Shadow rays determine light visibility
5. **Reflection**: Recursive ray bouncing for reflective materials

### Material System
- **Solid Colors**: Basic colored materials
- **Checkerboard Textures**: Procedural 3D patterns
- **Physical Properties**: Roughness, specularity, and reflectivity

### Performance
- **Optimized Builds**: Uses `opt-level = 3` even in debug mode
- **Real-time Rendering**: 800x600 resolution
- **Efficient Intersection**: Axis-aligned bounding box algorithm for cubes

## Customization

### Changing the Scene
Edit the `create_scene()` function in `main.rs`:
- Add more cubes or planes
- Modify materials and textures
- Adjust lighting positions
- Change background colors

### Camera Control
Modify the camera parameters in `main.rs`:
```rust
let camera = Camera::new(
    Vec3::new(3.0, 4.0, 2.0),      // Camera position
    Vec3::new(0.0, -0.5, -3.0),    // Look-at point
    Vec3::up(),                     // Up vector
    45.0,                           // Field of view
    aspect_ratio,                   // Aspect ratio
);
```

### Adding New Geometry
Implement the `Hittable` trait for new primitive types in `geometry.rs`.

## Educational Value

This raytracer serves as a learning tool for:
- **Computer Graphics**: Ray-object intersection algorithms
- **Linear Algebra**: 3D vector operations and transformations
- **Lighting Models**: Phong shading and shadow calculation
- **Rust Programming**: Modern systems programming concepts
- **Software Architecture**: Clean modular design

## License

This project is intended for educational purposes.

## Acknowledgments

- Built with [Raylib](https://www.raylib.com/) for graphics
- Inspired by classic raytracing literature
- Rust implementation for memory safety and performance
