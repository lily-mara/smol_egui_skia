use egui::{include_image, CentralPanel};
use skia_safe::EncodedImageFormat;
use smol_egui_skia::{rasterize, RasterizeOptions};
use std::fs::File;
use std::io::Write;

pub fn main() {
    let mut surface = rasterize(
        (460, 307),
        |ctx| {
            egui_extras::install_image_loaders(&ctx);
            CentralPanel::default().show(&ctx, |ui| {
                ui.image(include_image!("assets/ferris.jpg"));
            });
        },
        Some(RasterizeOptions {
            pixels_per_point: 1.0,
            frames_before_screenshot: 20,
        }),
    );

    let data = surface
        .image_snapshot()
        .encode_to_data(EncodedImageFormat::PNG)
        .expect("Failed to encode image");

    File::create("output.png")
        .unwrap()
        .write_all(&data)
        .unwrap();

    println!("wrote output.png");
}
