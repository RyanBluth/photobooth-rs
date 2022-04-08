use eframe::epaint::ColorImage;
use gphoto::Camera;
use v4l::buffer::Type;
use v4l::io::mmap::Stream;
use v4l::io::traits::CaptureStream;
use v4l::video::Capture;
use v4l::Device;
use v4l::FourCC;

use std::path::Path;

use eframe::{egui, epi};

fn main() {

    let mut context = gphoto::Context::new().unwrap();

    let mut camera = gphoto::Camera::autodetect(&mut context).unwrap();
 
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        Box::new(MyApp {
            camera: camera,
            camera_context: context,
        }),
        options,
    );
}

struct MyApp {
    camera: Camera,
    camera_context: gphoto::Context
}

impl<'a> epi::App for MyApp {
    fn name(&self) -> &str {
        "Show an image with eframe/egui"
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &epi::Frame) {
        let caputre = self.camera.capture_preview(&mut self.camera_context).unwrap();
        let image = image::load_from_memory(&caputre).unwrap();
        let size = [image.width() as _, image.height() as _];
        let image_buffer = image.to_rgba8();
        let pixels = image_buffer.as_flat_samples();
        let c_image = ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());

        let texture = ctx.load_texture("frame", c_image);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("This is an image:");

            ui.add(egui::Image::new(&texture, texture.size_vec2()));
        });

        ctx.request_repaint();
    }
}
