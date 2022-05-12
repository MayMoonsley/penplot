use crate::color::Color;
use std::cmp::Ordering;

// trait for drawing canvases, allowing us to abstract over drawing SVGs and PNGs
// the trait only exposes things the program state cares about, allowing it to stop worrying about implementation
pub trait DrawingCanvas {
    // create a new canvas given these params
    fn init(width: usize, height: usize, pen_x: f32, pen_y: f32) -> Self;

    // put pen on a specific coordinate
    fn move_pen_to(&mut self, x: f32, y: f32);

    // draw a single pixel / circle
    fn blot(&mut self, x: f32, y: f32);

    // set pen color
    fn set_color(&mut self, color: Color);

    // save to a file
    fn save(&self, filename: &str);
}

// raster graphics canvas
pub struct PixelCanvas {
    width: usize,
    height: usize,
    pen_x: f32,
    pen_y: f32,
    pen_color: Color,
    buffer: Vec<Color>
}

impl PixelCanvas {
    fn draw_pixel_f(&mut self, x: f32, y: f32) {
        self.draw_pixel_i(x.round() as isize, y.round() as isize);
    }

    fn draw_pixel_i(&mut self, x: isize, y: isize) {
        let w = self.width as isize;
        let h = self.height as isize;
        if x < 0 || y < 0 || x >= w || y >= h {
            // do nothing, since we're off the page
        } else {
            let index = (x + y * w) as usize;
            self.buffer[index] = Color::overlay(self.pen_color, self.buffer[index]);
        }
    }

    // bresenham's line algorithm
    fn plot_line(&mut self, mut x0: isize, mut y0: isize, x1: isize, y1: isize) {
        let dx = (x1 - x0).abs();
        let sx = match x0.cmp(&x1) {
            Ordering::Less => 1,
            _ => -1,
        };
        let dy = -(y1 - y0).abs();
        let sy = match y0.cmp(&y1) {
            Ordering::Less => 1,
            _ => -1,
        };
        let mut err = dx + dy;
        loop {
            self.draw_pixel_i(x0, y0);
            if x0 == x1 && y0 == y1 {
                break;
            }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x0 += sx;
            }
            if e2 <= dx {
                err += dx;
                y0 += sy;
            }
        }
    }
}

impl DrawingCanvas for PixelCanvas {
    fn init(width: usize, height: usize, pen_x: f32, pen_y: f32) -> Self {
        PixelCanvas {
            width, height, pen_x, pen_y,
            pen_color: Color::transparent(),
            buffer: vec![Color::transparent(); width * height]
        }
    }

    fn move_pen_to(&mut self, new_x: f32, new_y: f32) {
        if self.pen_color != Color::transparent() {
            self.plot_line(
                self.pen_x.round() as isize,
                self.pen_y.round() as isize,
                new_x.round() as isize,
                new_y.round() as isize,
            );
        }
        self.pen_x = new_x;
        self.pen_y = new_y;
    }

    fn blot(&mut self, x: f32, y: f32) {
        // TODO: pen width?
        self.draw_pixel_f(x, y);
    }

    fn set_color(&mut self, color: Color) {
        self.pen_color = color;
    }

    fn save(&self, filename: &str) {
        let mut bytes: Vec<u8> = vec![0; self.width * self.height * 4];
        for index in 0..self.width * self.height {
            let Color(r, g, b, a) = self.buffer[index];
            bytes[index * 4] = r;
            bytes[index * 4 + 1] = g;
            bytes[index * 4 + 2] = b;
            bytes[index * 4 + 3] = a;
        }
        // TODO: return this error
        image::save_buffer(
            filename,
            &bytes,
            self.width as u32,
            self.height as u32,
            image::ColorType::Rgba8,
        ).unwrap();
    }
}