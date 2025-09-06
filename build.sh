#!/bin/bash
# Build script for Clean Raytracer

echo "Building Clean Raytracer..."

# Build in release mode for optimal performance
cargo build --release

if [ $? -eq 0 ]; then
    echo "✅ Build successful!"
    echo "Run with: cargo run --release"
    echo "Or execute: ./target/release/raytracer_clean"
else
    echo "❌ Build failed!"
    exit 1
fi
