// use std::f32::consts::E;

use error_iter::ErrorIter as _;
use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::PhysicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

const WIDTH: u32 = 1000;
const HEIGHT: u32 = 1000;
const W2: f32 = WIDTH as f32 / 2.;
const H2: f32 = HEIGHT as f32 / 2.;

struct World {}

fn dx_dy(x: f32, y: f32) -> f32 {
    // 2. * x - y
    // 2. - (x * y)
    // x + y
    // -x / y
    // (3. * x.powi(2) + 1.) / (2. * y)
    x + y
}

fn sigmoid(x: f32) -> f32 {
    // 1. / (1. + E.powf(-x))
    (x / 3.).tanh()
}

fn main() -> Result<(), Error> {
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = PhysicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Slope Fields")
            .with_inner_size(size)
            .with_resizable(false)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };
    let mut world = World::new();

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            world.draw(pixels.frame_mut());
            if let Err(err) = pixels.render() {
                log_error("pixels.render", err);
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.close_requested() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Resize the window
            // if let Some(size) = input.window_resized() {
            //     if let Err(err) = pixels.resize_surface(size.width, size.height) {
            //         log_error("pixels.resize_surface", err);
            //         *control_flow = ControlFlow::Exit;
            //         return;
            //     }
            // }

            // Update internal state and request a redraw
            world.update();
            window.request_redraw();
        }
    });
}

fn log_error<E: std::error::Error + 'static>(method_name: &str, err: E) {
    error!("{method_name}() failed: {}", err);
    for source in err.sources().skip(1) {
        error!("  Caused by: {source}");
    }
}

impl World {
    fn new() -> Self {
        Self {}
    }

    fn update(&mut self) {}

    fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = (i % WIDTH as usize) as i32;
            let y = (i / WIDTH as usize) as i32;

            let rx = ((x as f32 - W2) / W2) * 10.;
            let ry = (-(y as f32 - H2) / H2) * 10.;

            let m = dx_dy(rx, ry);

            let g = sigmoid(m);
            let mut rgba = [0, 0, 0, 0xff];
            if g > 0.0 {
                rgba[0] = (g * 255.0) as u8
            } else {
                rgba[2] = (-g * 255.0) as u8
            }

            // if m.abs() >= 0.95 && m.abs() <= 1.05 {
            //     rgba = [0xff, 0xff, 0xff, 0xff]
            // }

            pixel.copy_from_slice(&rgba);
        }
    }
}
