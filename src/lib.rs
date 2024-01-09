use std::f32::consts::PI;

pub mod world;

// THIS CLOSURE CONTAINS THE DIFFERENTIAL EQ. USED!!!
// SUBSTITUTE THIS WITH ANYTHING YOU LIKE!!!
const DY_DX: fn(f32, f32) -> f32 = |x, y| {
    // x / y
    // x + y
    // x % y
    x.sin() + 2. * y.cos()
    // x.sin() / y.cos()
};

pub const WIDTH: u32 = 1000;
pub const HEIGHT: u32 = 1000;

// Half of width & height for convenience
const W2: f32 = WIDTH as f32 / 2.;
const H2: f32 = HEIGHT as f32 / 2.;

// Convert from pixel coordinates to coordinates on the grid
pub fn pixel_to_grid_space(x: i32, y: i32) -> (f32, f32) {
    let new_x = ((x as f32 - W2) / W2) * 10.;
    let new_y = (-(y as f32 - H2) / H2) * 10.;
    (new_x, new_y)
}

// Convert from coordinates on the grid to pixel coordinate
pub fn grid_to_pixel_space(x: f32, y: f32) -> (i32, i32) {
    let new_x = (((x / 10.) * W2) + W2) as i32;
    let new_y = ((-(y / 10.) * H2) + H2) as i32;
    (new_x, new_y)
}

// Draw a slope line given a grid point, length, and whether to draw an arrow
pub fn draw_slope_line(frame: &mut [u8], x: f32, y: f32, len: f32, draw_arrow: bool) {
    let (px, py) = grid_to_pixel_space(x, y);
    let m = DY_DX(x, y);
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
        draw_line(frame, px + dx, py + dy, px + dx + dx2, py + dy + dy2);

        let a3 = a - offset;
        let dx3 = (r2 * a3.cos()) as i32;
        let dy3 = (r2 * a3.sin()) as i32;
        draw_line(frame, px + dx, py + dy, px + dx + dx3, py + dy + dy3);
    }
}

// Simple line drawing algorithm
pub fn draw_line(frame: &mut [u8], x1: i32, y1: i32, x2: i32, y2: i32) {
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

        if x1 == x2 && y1 == y2 {
            break;
        }
        let e2 = error * 2.;
        if e2 >= dy {
            if x1 == x2 {
                break;
            }
            error += dy;
            x1 += sx;
        }
        if e2 <= dx {
            if y1 == y2 {
                break;
            }
            error += dx;
            y1 += sy;
        }
    }
}

// Sigmoid function for flattening values as they approach infinity
// (as they so often seem to do)
pub fn sigmoid(x: f32) -> f32 {
    // 1. / (1. + E.powf(-x))
    (x * 0.25).tanh()
}
