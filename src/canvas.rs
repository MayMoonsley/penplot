use crate::color::Color;
use std::cmp::{self, Ordering};

// trait for drawing canvases, allowing us to abstract over drawing SVGs and PNGs
// the trait only exposes things the program state cares about, allowing it to stop worrying about implementation
pub trait DrawingCanvas {
    // put pen on a specific coordinate
    fn move_pen_to(&mut self, x: f32, y: f32);

    // draw a single pixel / circle
    fn blot(&mut self, x: f32, y: f32);

    // set pen color
    fn set_color(&mut self, color: Color);
}

// trait for canvases that can be saved
// this is distinct from DrawingCanvas because of SizingCanvas
pub trait SaveableCanvas {
    // save to a file
    fn save(&self, filename: &str);
}

// raster graphics canvas
pub struct PixelCanvas {
    width: usize,
    height: usize,
    x_offset: isize,
    y_offset: isize,
    pen_x: f32,
    pen_y: f32,
    pen_color: Color,
    buffer: Vec<Color>
}

impl PixelCanvas {
    pub fn new(width: usize, height: usize, x_offset: isize, y_offset: isize) -> Self {
        PixelCanvas {
            width, height, x_offset, y_offset,
            pen_x: 0.0,
            pen_y: 0.0,
            pen_color: Color::transparent(),
            buffer: vec![Color::transparent(); width * height]
        }
    }

    fn draw_pixel_f(&mut self, x: f32, y: f32) {
        self.draw_pixel_i(x.round() as isize, y.round() as isize);
    }

    fn draw_pixel_i(&mut self, x: isize, y: isize) {
        let w = self.width as isize;
        let h = self.height as isize;
        let x = x + self.x_offset;
        let y = y + self.y_offset;
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
}

impl SaveableCanvas for PixelCanvas {
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

// "canvas" that merely keeps track of the bounding box of the drawing
// this can be used to compute offsets / necessary width
pub struct SizingCanvas {
    min_x: isize,
    min_y: isize,
    max_x: isize,
    max_y: isize
}

impl SizingCanvas {
    pub fn new() -> Self {
        SizingCanvas {
            min_x: 0,
            min_y: 0,
            max_x: 0,
            max_y: 0
        }
    }

    pub fn dimensions(&self) -> (usize, usize) {
        if self.max_x < self.min_x || self.max_y < self.min_y {
            panic!("SizingCanvas min < max");
        }
        // these values add one to avoid off-by-one errors (if the image has min_x = 512, max_x = 512, it's 1 pixel wide)
        ((self.max_x - self.min_x + 1) as usize, (self.max_y - self.min_y + 1) as usize)
    }

    pub fn offsets(&self) -> (isize, isize) {
        (-self.min_x, -self.min_y)
    }

    fn update_values(&mut self, new_x: isize, new_y: isize) {
        // update mins
        self.min_x = cmp::min(self.min_x, new_x);
        self.min_y = cmp::min(self.min_y, new_y);
        // update maxes
        self.max_x = cmp::max(self.max_x, new_x);
        self.max_y = cmp::max(self.max_y, new_y);
    }
}

impl DrawingCanvas for SizingCanvas {
    fn move_pen_to(&mut self, x: f32, y: f32) {
        self.update_values(x.round() as isize, y.round() as isize);
    }

    // the below methods are no-ops since color / blotting doesn't matter
    fn blot(&mut self, _x: f32, _y: f32) {

    }

    fn set_color(&mut self, _color: Color) {

    }
}