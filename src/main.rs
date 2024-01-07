use pixels::{Pixels, SurfaceTexture};
use slope_fields::world::World;
use slope_fields::{HEIGHT, WIDTH};
use winit::dpi::PhysicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

fn main() {
    // Create new world instance which holds internal state of the program
    let mut world = World::new();

    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = PhysicalSize::new(WIDTH, HEIGHT);
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
        Pixels::new(WIDTH, HEIGHT, surface_texture).unwrap()
    };

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            world.draw(pixels.frame_mut());
            if let Err(err) = pixels.render() {
                eprintln!("{}", err);
                control_flow.set_exit();
                return;
            }
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.close_requested() {
                control_flow.set_exit();
                return;
            }

            if let Some(pos) = input.mouse() {
                world.mouse_pos = pos;
            }

            // Update internal state and request a redraw
            // world.update();
            window.request_redraw();
        }
    });
}
