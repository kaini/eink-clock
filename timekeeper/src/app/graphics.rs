// For triangle rasterization ideas see http://www.sunshine2k.de/coding/java/TriangleRasterization/TriangleRasterization.html
// For Bresenham's line rasterization algorithm see https://de.wikipedia.org/wiki/Bresenham-Algorithmus#Kompakte_Variante
use devices::flash::Font;
use core::intrinsics::{sqrtf32, roundf32};

pub enum HorizontalAlign { 
    LEFT,                  
    CENTER,                
    RIGHT,                 
}

pub fn render_image<SP: Fn(i32, i32)>(set_pixel: &SP,
                                      image: &[u8], image_w: i32, _image_h: i32,
                                      source_x: i32, source_y: i32,
                                      dest_x: i32, dest_y: i32, w: i32, h: i32) {
    for x in 0..w {
        for y in 0..h {
            let image_bit = ((source_y + y) * image_w + source_x + x) as usize;
            if image[image_bit / 8] & (0b1000_0000 >> (image_bit % 8)) == 0 {
                set_pixel(dest_x + x, dest_y + y);
            }
        }
    }
}



pub fn render_triangle<SP: Fn(i32, i32)>(set_pixel: &SP, ax: i32, ay: i32, bx: i32, by: i32, cx: i32, cy: i32) {
    // Make a the left point, b the center point and c the right point.
    if bx < ax || cx < ax {
        if bx < cx {
            return render_triangle(set_pixel, bx, by, ax, ay, cx, cy);
        } else {
            return render_triangle(set_pixel, cx, cy, bx, by, ax, ay);
        }
    }
    if cx < bx {
        return render_triangle(set_pixel, ax, ay, cx, cy, bx, by);
    }

    // Render the first half of the triangle, i.e., consume ab.
    let mut ab = Bresenham::new(ax, ay, bx, by);
    let mut ac = Bresenham::new(ax, ay, cx, cy);
    let mut prev_x = ax;
    let mut ab_done = false;
    let mut ac_done = false;
    while !(ab_done || ac_done) {
        // Advance ab until the next x step.
        let mut ab_step = ab.step();
        while !ab_step.last && ab_step.x == prev_x {
            set_pixel(ab_step.x, ab_step.y);
            ab_step = ab.step();
        }
        ab_done |= ab_step.last;

        // Advance ac until the next x step.
        let mut ac_step = ac.step();
        while !ac_step.last && ac_step.x == prev_x {
            set_pixel(ac_step.x, ac_step.y);
            ac_step = ac.step();
        }
        ac_done |= ac_step.last;

        // ab and ac are now advanced to the same x.
        fill_y(set_pixel, ab_step.x, ab_step.y, ac_step.y);
        prev_x = ab_step.x;
    }

    // Finish ab if it is not completely finished.
    if !ab_done {
        loop {
            let ab_step = ab.step();
            set_pixel(ab_step.x, ab_step.y);
            if ab_step.last {
                break;
            }
        }
    }

    // Draw the second half of the triangle, i.e., consume bc.
    let mut bc = Bresenham::new(bx, by, cx, cy);
    let mut bc_done = false;
    while !(ac_done || bc_done) {
        // Advance bc until the next x step.
        let mut bc_step = bc.step();
        while !bc_step.last && bc_step.x == prev_x {
            set_pixel(bc_step.x, bc_step.y);
            bc_step = bc.step();
        }
        bc_done |= bc_step.last;

        // Advance ac until the next x step.
        let mut ac_step = ac.step();
        while !ac_step.last && ac_step.x == prev_x {
            set_pixel(ac_step.x, ac_step.y);
            ac_step = ac.step();
        }
        ac_done |= ac_step.last;

        // ab and ac are now advanced to the same x.
        fill_y(set_pixel, bc_step.x, bc_step.y, ac_step.y);
        prev_x = bc_step.x;
    }

    // Finish bc if it is not completely finished.
    if !bc_done {
        loop {
            let bc_step = bc.step();
            set_pixel(bc_step.x, bc_step.y);
            if bc_step.last {
                break;
            }
        }
    }

    // Finish ac if it is not completely finished.
    if !ac_done {
        loop {
            let ac_step = ac.step();
            set_pixel(ac_step.x, ac_step.y);
            if ac_step.last {
                break;
            }
        }
    }
}

pub fn render_line<SP: Fn(i32, i32)>(set_pixel: &SP, x0: i32, y0: i32, x1: i32, y1: i32, width: i32) {
    // Get a normal vector by flipping x and y and negating x thereafter.
    let norm_x = (y0 - y1) as f32;
    let norm_y = (x1 - x0) as f32;
    let norm_factor = (width as f32) / (2.0 * sqrt(norm_x * norm_x + norm_y * norm_y));
    let norm_x = round(norm_x * norm_factor) as i32;
    let norm_y = round(norm_y * norm_factor) as i32;
    render_triangle(set_pixel, x0 - norm_x, y0 - norm_y, x1 - norm_x, y1 - norm_y, x0 + norm_x, y0 + norm_y);
    render_triangle(set_pixel, x0 + norm_x, y0 + norm_y, x1 + norm_x, y1 + norm_y, x1 - norm_x, y1 - norm_y);
}

pub fn render_text<SP: Fn(i32, i32)>(set_pixel: &SP, text: &str, font: &Font<'static>, x: i32, y: i32, halign: HorizontalAlign) {
    let mut x_length = 0;
    for c in text.chars() {
        if let Ok(index) = font.chars.binary_search_by_key(&(c as u32), |ref chr| chr.chr) {
            x_length += font.chars[index].xadvance as i32;
        }
    }

    let mut x_at = match halign {
        HorizontalAlign::LEFT => { x },
        HorizontalAlign::CENTER => { x - x_length / 2 },
        HorizontalAlign::RIGHT => { x - x_length }, 
    };
    let mut prev_char = 0;
    for c in text.chars() {
        if let Ok(index) = font.chars.binary_search_by_key(&(c as u32), |ref chr| chr.chr) {
            x_at += if let Ok(index) = font.kerning_pairs.binary_search_by_key(&(prev_char, c as u32), |ref k| k.pair) {
                font.kerning_pairs[index].amount as i32
            } else {
                0
            };
            let ref chr = font.chars[index];
            render_image(
                set_pixel,
                font.texture, font.texture_w, font.texture_h,
                chr.x as i32, chr.y as i32, 
                x_at + chr.xoffset as i32, y + chr.yoffset as i32, chr.width as i32, chr.height as i32);
            x_at += chr.xadvance as i32;
            prev_char = chr.chr;
        }
    }
}

fn fill_y<SP: Fn(i32, i32)>(set_pixel: &SP, x: i32, y0: i32, y1: i32) {
    if y0 < y1 {
        for y in y0..(y1 + 1) {
            set_pixel(x, y);
        }
    } else {
        for y in y1..(y0 + 1) {
            set_pixel(x, y);
        }
    }
}

struct Bresenham {
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
    dx: i32,
    sx: i32,
    dy: i32,
    sy: i32,
    err: i32,
    first: bool,
}

struct BresenhamStep {
    x: i32,
    y: i32,
    last: bool,
}

impl Bresenham {
    fn new(x0: i32, y0: i32, x1: i32, y1: i32) -> Bresenham {
        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        Bresenham {
            x0: x0,
            y0: y0,
            x1: x1,
            y1: y1,
            dx: dx,
            sx: if x0 < x1 { 1 } else { -1 },
            dy: dy,
            sy: if y0 < y1 { 1 } else { -1 },
            err: dx + dy,
            first: true,
        }
    }

    fn step(&mut self) -> BresenhamStep {
        if self.first {
            self.first = false;
        } else {
            let e2 = 2 * self.err;
            if e2 > self.dy {
                self.err += self.dy;
                self.x0 += self.sx;
            }
            if e2 < self.dx {
                self.err += self.dx;
                self.y0 += self.sy;
            }
        }
        BresenhamStep { x: self.x0, y: self.y0, last: self.x0 == self.x1 && self.y0 == self.y1 }
    }
}

fn round(f: f32) -> f32 {
    unsafe { roundf32(f) }
}

fn sqrt(f: f32) -> f32 {
    unsafe { sqrtf32(f) }
}
