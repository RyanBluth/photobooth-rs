use eframe::epaint::ColorImage;
use v4l::buffer::Type;
use v4l::io::mmap::Stream;
use v4l::io::traits::CaptureStream;
use v4l::video::Capture;
use v4l::Device;
use v4l::FourCC;

use eframe::{egui, epi};

fn main() {
    // Create a new capture device with a few extra parameters
    let mut dev = Device::new(0).expect("Failed to open device");

    // Let's say we want to explicitly request another format
    let mut fmt = dev.format().expect("Failed to read format");
    fmt.width = 1280;
    fmt.height = 720;
    fmt.fourcc = FourCC::new(b"MJPG");
    dev.set_format(&fmt).expect("Failed to write format");

    // The actual format chosen by the device driver may differ from what we
    // requested! Print it out to get an idea of what is actually used now.
    println!("Format in use:\n{}", fmt);

    // Now we'd like to capture some frames!
    // First, we need to create a stream to read buffers from. We choose a
    // mapped buffer stream, which uses mmap to directly access the device
    // frame buffer. No buffers are copied nor allocated, so this is actually
    // a zero-copy operation.

    // To achieve the best possible performance, you may want to use a
    // UserBufferStream instance, but this is not supported on all devices,
    // so we stick to the mapped case for this example.
    // Please refer to the rustdoc docs for a more detailed explanation about
    // buffer transfers.

    // Create the stream, which will internally 'allocate' (as in map) the
    // number of requested buffers for us.
    let mut stream = Stream::with_buffers(&mut dev, Type::VideoCapture, 4)
        .expect("Failed to create buffer stream");

    // At this point, the stream is ready and all buffers are setup.
    // We can now read frames (represented as buffers) by iterating through
    // the stream. Once an error condition occurs, the iterator will return
    // None.
    //loop {
    let (buf, meta) = stream.next().unwrap();
    println!(
        "Buffer size: {}, seq: {}, timestamp: {}",
        buf.len(),
        meta.sequence,
        meta.timestamp
    );

    // To process the captured data, you can pass it somewhere else.
    // If you want to modify the data or extend its lifetime, you have to
    // copy it. This is a best-effort tradeoff solution that allows for
    // zero-copy readers while enforcing a full clone of the data for
    // writers.
    //}

    let options = eframe::NativeOptions::default();
    eframe::run_native(
        Box::new(MyApp {
            image_stream: stream,
        }),
        options,
    );
}

struct MyApp<'a> {
    image_stream: Stream<'a>,
}

impl<'a> epi::App for MyApp<'a> {
    fn name(&self) -> &str {
        "Show an image with eframe/egui"
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &epi::Frame) {
        let image = image::load_from_memory(self.image_stream.next().unwrap().0).unwrap();
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
