use raylib::prelude::*;

#[derive(Debug, Clone)]
pub struct Texture {
    pub image: Image,
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<Vector3>, // Store pixel colors for easy access
}

impl Texture {
    pub fn load_from_file(filename: &str) -> Result<Self, String> {
        match Image::load_image(filename) {
            Ok(mut image) => {
                let width = image.width as u32;
                let height = image.height as u32;
                
                // Extract pixel data
                let mut pixels = Vec::with_capacity((width * height) as usize);
                for y in 0..height {
                    for x in 0..width {
                        let color = image.get_color(x as i32, y as i32);
                        pixels.push(Vector3::new(
                            color.r as f32 / 255.0,
                            color.g as f32 / 255.0,
                            color.b as f32 / 255.0,
                        ));
                    }
                }
                
                Ok(Texture {
                    image,
                    width,
                    height,
                    pixels,
                })
            }
            Err(_) => Err(format!("Failed to load texture: {}", filename)),
        }
    }

    pub fn create_dummy(width: u32, height: u32) -> Self {
        // Create a checkerboard pattern as a dummy texture
        let mut image = Image::gen_image_color(width as i32, height as i32, Color::WHITE);
        let mut pixels = Vec::with_capacity((width * height) as usize);
        
        // Create checkerboard pattern
        let checker_size = 16;
        for y in 0..height {
            for x in 0..width {
                let checker_x = (x / checker_size) % 2;
                let checker_y = (y / checker_size) % 2;
                
                let color = if (checker_x + checker_y) % 2 == 0 {
                    Color::new(200, 200, 200, 255) // Light gray
                } else {
                    Color::new(100, 100, 100, 255) // Dark gray
                };
                
                image.draw_pixel(x as i32, y as i32, color);
                
                // Store pixel in our array
                pixels.push(Vector3::new(
                    color.r as f32 / 255.0,
                    color.g as f32 / 255.0,
                    color.b as f32 / 255.0,
                ));
            }
        }

        Texture {
            image,
            width,
            height,
            pixels,
        }
    }

    pub fn sample(&self, u: f32, v: f32) -> Vector3 {
        // Clamp UV coordinates to [0, 1] and handle wrapping
        let u_clamped = u.fract().abs();
        let v_clamped = (1.0 - v).fract().abs(); // Flip V coordinate for correct orientation
        
        // Convert UV to pixel coordinates
        let x = (u_clamped * (self.width - 1) as f32) as u32;
        let y = (v_clamped * (self.height - 1) as f32) as u32;
        
        // Ensure coordinates are within bounds
        let x = x.min(self.width - 1);
        let y = y.min(self.height - 1);
        
        // Calculate pixel index
        let index = (y * self.width + x) as usize;
        
        // Return the pixel color
        if index < self.pixels.len() {
            self.pixels[index]
        } else {
            // Fallback color if index is out of bounds
            Vector3::new(1.0, 0.0, 1.0) // Magenta as error indicator
        }
    }
}