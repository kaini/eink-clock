use collections::vec::Vec;
use alloc::boxed::Box;
use devices::flash::Font;

const GRID_SIZE: usize = 256;

#[derive(Debug, PartialEq, Eq)]
pub enum Color {
    TRANSPARENT,
    BLACK,
}

trait Renderable {
    fn render_pixel(&self, x: i32, y: i32) -> Color;
    fn bounding_box(&self) -> (i32, i32, i32, i32);  // Returns (x, y, w, h)
}

struct Image {
    source: &'static [u8],
    source_x: i32,
    source_y: i32,
    source_width: i32,
    source_height: i32,
    dest_x: i32,
    dest_y: i32,
    width: i32,
    height: i32,
}

impl Renderable for Image {
    fn render_pixel(&self, x: i32, y: i32) -> Color {
        if self.dest_x <= x && x < self.dest_x + self.width &&
           self.dest_y <= y && y < self.dest_y + self.height {
            let source_bit = ((self.source_y + y - self.dest_y) * self.source_width + self.source_x + x - self.dest_x) as usize;
            if self.source[source_bit / 8] & (0b1000_0000 >> (source_bit % 8)) == 0 {
                Color::BLACK
            } else {
                Color::TRANSPARENT
            }
        } else {
            Color::TRANSPARENT
        }
    }

    fn bounding_box(&self) -> (i32, i32, i32, i32) {
        (self.dest_x, self.dest_y, self.width, self.height)
    }
}

pub enum HorizontalAlign {
    LEFT,
    CENTER,
    RIGHT,
}

pub struct Graphic {
    elements: Vec<Box<Renderable>>,
    width: i32,
    height: i32,
    grid: Vec<Vec<usize>>,
}

impl Graphic {
    pub fn new(w: i32, h: i32) -> Graphic {
        Graphic {
            elements: Vec::new(),
            width: w,
            height: h,
            grid: vec![Vec::new(); (w as usize / GRID_SIZE + 1) * (h as usize / GRID_SIZE + 1)],
        }
    }

    pub fn add_text(&mut self, text: &str, font: &Font<'static>, x: i32, y: i32, halign: HorizontalAlign) {
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
                let kerning = if let Ok(index) = font.kerning_pairs.binary_search_by_key(&(prev_char, c as u32), |ref k| k.pair) {
                    font.kerning_pairs[index].amount as i32
                } else {
                    0
                };
                let ref chr = font.chars[index];
                self.add_image(
                    font.texture, chr.x as i32, chr.y as i32, font.texture_w, font.texture_h,
                    x_at + chr.xoffset as i32 + kerning, y + chr.yoffset as i32, chr.width as i32, chr.height as i32);
                x_at += chr.xadvance as i32;
                prev_char = chr.chr;
            }
        }
    }

    pub fn add_image(&mut self,
            source: &'static [u8], source_x: i32, source_y: i32, source_width: i32, source_height: i32,
            dest_x: i32, dest_y: i32, width: i32, height: i32) {
        self.add_element(Box::new(Image {
            source: source,
            source_x: source_x,
            source_y: source_y,
            source_width: source_width,
            source_height: source_height,
            dest_x: dest_x,
            dest_y: dest_y,
            width: width,
            height: height,
        }));
    }

    fn xy_to_grid_xy(&self, x: i32, y: i32) -> (usize, usize) {
        ((x as usize / GRID_SIZE), (y as usize / GRID_SIZE))
    }

    fn grid_xy_to_grid_index(&self, gx: usize, gy: usize) -> usize {
        gy * (self.width as usize / GRID_SIZE) + gx
    }

    fn xy_to_grid_index(&self, x: i32, y: i32) -> usize {
        let (gx, gy) = self.xy_to_grid_xy(x, y);
        self.grid_xy_to_grid_index(gx, gy)
    }

    fn add_element(&mut self, renderable: Box<Renderable>) {
        let index = self.elements.len();
        
        // Clip the bounding box
        let (mut bx, mut by, mut bw, mut bh) = renderable.bounding_box();
        if bx < 0 {
            bw += bx;
            bx = 0;
        }
        if by < 0 {
            bh += by;
            by = 0;
        }
        if bx + bw > self.width {
            bw = self.width - bx;
        }
        if by + bh > self.height {
            bh = self.height - by;
        }
        if bw <= 0 || bh <= 0 {
            return;
        }

        let (gx0, gy0) = self.xy_to_grid_xy(bx, by);
        let (gx1, gy1) = self.xy_to_grid_xy(bx + bw - 1, by + bh - 1);
        for gx in gx0..(gx1 + 1) {
            for gy in gy0..(gy1 + 1) {
                let grid_index = self.grid_xy_to_grid_index(gx, gy);
                self.grid[grid_index].push(index);
            }
        }

        self.elements.push(renderable);
    }

    pub fn render_pixel(&self, x: i32, y: i32) -> Color {
        let index = self.xy_to_grid_index(x, y);
        for &element_index in &self.grid[index] {
            if self.elements[element_index].render_pixel(x, y) == Color::BLACK {
                return Color::BLACK;
            }
        }
        Color::TRANSPARENT
    }
}
