use std::f64::consts::PI;
use std::io::stdin;
use pixels::{Pixels, SurfaceTexture};
use winit::dpi::PhysicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

const WIDTH: u32 = 1000;
const HEIGHT: u32 = 1000;
const W2: f64 = WIDTH as f64 / 2.;
const H2: f64 = HEIGHT as f64 / 2.;

struct World<F>
where
    F: Fn(f64, f64) -> f64
{
    mouse_pos: (f32, f32),
    func: F
}

fn sigmoid(x: f64) -> f64 {
    // 1. / (1. + E.powf(-x))
    (x * 0.25).tanh()
}

fn main()  {
    println!("Enter differential: ");
    let mut input_equation = String::new();
    stdin().read_line(&mut input_equation).unwrap();

    let expr = input_equation.trim().parse::<meval::Expr>().expect("Error parsing differential equation!");
    let func = expr.bind2("x", "y").unwrap();

    let mut world = World::new(func);

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
            world.update();
            window.request_redraw();
        }
    });
}

impl<F> World<F>
where
    F: Fn(f64, f64) -> f64
{
    fn new(func: F) -> Self {
        Self {
            mouse_pos: (0., 0.),
            func
        }
    }

    fn dx_dy(&self, x: f64, y: f64) -> f64 {
        (self.func)(x, y)
    }

    fn update(&mut self) {}

    fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = (i % WIDTH as usize) as i32;
            let y = (i / WIDTH as usize) as i32;
            let (rx, ry) = pixel_to_grid_space(x, y);

            let m = self.dx_dy(rx, ry);

            let g = sigmoid(m);
            let mut rgba = [0, 0, 0, 0xff];
            if g > 0.0 {
                rgba[0] = (g * 255.0) as u8
            } else {
                rgba[2] = (-g * 255.0) as u8
            }

            pixel.copy_from_slice(&rgba);
        }

        for i in 0..=18 {
            for j in 0..=18 {
                let rx = (i - 9) as f64;
                let ry = (j - 9) as f64;
                draw_slope_line(frame, rx, ry, 40., true, &self);
            }
        }

        let x = self.mouse_pos.0 as i32;
        let y = self.mouse_pos.1 as i32;
        let (rx, ry) = pixel_to_grid_space(x, y);
        draw_slope_line(frame, rx, ry, 100., true, &self);
    }
}

fn pixel_to_grid_space(x: i32, y: i32) -> (f64, f64) {
    let new_x = ((x as f64 - W2) / W2) * 10.;
    let new_y = (-(y as f64 - H2) / H2) * 10.;
    (new_x, new_y)
}

fn grid_to_pixel_space(x: f64, y: f64) -> (i32, i32) {
    let new_x = (((x / 10.) * W2) + W2) as i32;
    let new_y = ((-(y / 10.) * H2) + H2) as i32;
    (new_x, new_y)
}

fn draw_slope_line<F>(frame: &mut [u8], x: f64, y: f64, len: f64, draw_arrow: bool, world: &World<F>)
where
    F: Fn(f64, f64) -> f64
{
    let (px, py) = grid_to_pixel_space(x, y);
    let m = world.dx_dy(x, y);
    let a = m.atan();

    let r = len / 2.;

    let dx = (r * a.cos()) as i32;
    let dy = (r * a.sin()) as i32;

    draw_line(frame, px + dx, py + dy, px - dx, py - dy);

    if draw_arrow {
        let offset = 3. * PI / 4.;
        let a2 = a + offset;
        let r2 = 7.5;
        let dx2 = (r2 * a2.cos()) as i32;
        let dy2 = (r2 * a2.sin()) as i32;
        draw_line(frame, px+dx, py+dy, px+dx+dx2, py+dy+dy2);

        let a3 = a - offset;
        let dx3 = (r2 * a3.cos()) as i32;
        let dy3 = (r2 * a3.sin()) as i32;
        draw_line(frame, px+dx, py+dy, px+dx+dx3, py+dy+dy3);
    }
}

fn draw_line(frame: &mut [u8], x1: i32, y1: i32, x2: i32, y2: i32) {
    let mut x1 = x1 as f32;
    let mut y1 = y1 as f32;
    let x2 = x2 as f32;
    let y2 = y2 as f32;

    let dx = (x2 - x1).abs();
    let sx = if x1 < x2 { 1. } else { -1. };
    let dy = -(y2 - y1).abs();
    let sy = if y1 < y2 { 1. } else { -1. };
    let mut error = dx + dy;

    loop {
        let x = x1 as usize;
        let y = y1 as usize;
        if x < WIDTH as usize && y < HEIGHT as usize {
            let i = y * WIDTH as usize + x;
            let pixel = frame.chunks_exact_mut(4).nth(i).unwrap();
            pixel.copy_from_slice(&[255, 255, 255, 255]);
        }

        if x1 == x2 && y1 == y2 { break }
        let e2 = error * 2.;
        if e2 >= dy {
            if x1 == x2 { break }
            error += dy;
            x1 += sx;
        }
        if e2 <= dx {
            if y1 == y2 { break }
            error += dx;
            y1 += sy;
        }
    }
}