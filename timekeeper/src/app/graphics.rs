use collections::vec::Vec;
use alloc::boxed::Box;
use devices::flash::Font;
use core::cmp::{min, max};

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

// ax + by + c = 0
#[derive(Debug)]
struct Line {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    d: i32,

    a: i32,  // a = y2 - y1
    b: i32,  // b = x2 - x1
    c: i32,  // c = x2*y1 - y2*x1
    dist_test: i32,  // d²(a² + b²)

    minx_test: i32,  // min(x1, x2) * (a² + b²) + a*c
    maxx_test: i32,  // max(x1, x2) * (a² + b²) + a*c
    miny_test: i32,  // min(y1, y2) * (a² + b²)
    maxy_test: i32,  // max(y1, y2) * (a² + b²)
}

impl Renderable for Line {
    fn render_pixel(&self, x: i32, y: i32) -> Color {
        // First test the distance to the unbounded line
        //     |a*x + b*y + c| / sqrt(a² + b²) <= d
        // <=> (a*x + b*y + c)² / (a² + b²) <= d²
        // <=> (a*x + b*y + c)² <= d²(a² + b²)
        let q = self.a * x + self.b * y + self.c;
        // TODO The next two lines are using `as i64` instead of an overflow-checking
        //      multiply because of LLVM bug #00000.
        let q = q as i64 * q as i64;
        // Test if the point is too far away from the line
        if q > self.dist_test as i64 {
            return Color::TRANSPARENT;
        }
        // If the point is inside the (x1,y1)-(x2,y2) box it is surely on the
        // line, because the endings are never in this box.
        if min(self.x1, self.x2) <= x && x <= max(self.x1, self.x2) &&
           min(self.y1, self.y2) <= y && y <= max(self.y1, self.y2) {
            return Color::BLACK;
        }
        // Now test if the projection on the line is between the start and endpoint.
        //     min_x <= (b(b*x - a*y) - a*c) / (a² + b²) <= max_x
        // <=> min_x(a² + b²) <= b(b*x - a*y) - a*c <= max_x(a² + b²)
        // <=> min_x(a² + b²) + a*c <= b(b*x - a*y) <= max_x(a² + b²) + a*c
        // Similar for y:
        //     min_y <= (a(a*y - b*x) - b*c) / (a² + b²) <= max_y
        // <=> min_y(a² + b²) <= a(a*y - b*x) - b*c <= max_y(a² + b²)
        // <=> min_y(a² + b²) + b*c <= a(a*y - b*x) <= max_y(a² + b²) + b*c
        let x_test = self.b * (self.b * x - self.a * y);
        let y_test = self.a * (self.a * y - self.b * x);
        if self.minx_test <= x_test && x_test <= self.maxx_test &&
           self.miny_test <= y_test && y_test <= self.maxy_test {
            return Color::BLACK
        }
        // The pixel is not relevant for this line
        Color::TRANSPARENT
    }

    fn bounding_box(&self) -> (i32, i32, i32, i32) {
        let minx = min(self.x1, self.x2) - self.d - 1;
        let maxx = max(self.x1, self.x2) + self.d + 1;
        let miny = min(self.y1, self.y2) - self.d - 1;
        let maxy = max(self.y1, self.y2) + self.d + 1;
        (minx, miny, maxx - minx + 1, maxy - miny + 1)
    }
}

pub enum HorizontalAlign {
    LEFT,
    CENTER,
    RIGHT,
}

struct ElementEntry {
    value: i32,
    index: usize,
}

pub struct Graphic {
    elements: Vec<Box<Renderable>>,
    width: i32,
    height: i32,

    x_starts: Vec<ElementEntry>,
    x_ends: Vec<ElementEntry>,
    x_starts_at: usize,
    x_ends_at: usize,
    x_at: i32,
    x_active: Vec<usize>,

    y_at: i32,
}

impl Graphic {
    pub fn new(w: i32, h: i32) -> Graphic {
        Graphic {
            elements: Vec::new(),
            width: w,
            height: h,

            x_starts: Vec::new(),
            x_ends: Vec::new(),
            x_starts_at: 0,
            x_ends_at: 0,
            x_at: w - 1,
            x_active: Vec::new(),

            y_at: 0,
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
                x_at += if let Ok(index) = font.kerning_pairs.binary_search_by_key(&(prev_char, c as u32), |ref k| k.pair) {
                    font.kerning_pairs[index].amount as i32
                } else {
                    0
                };
                let ref chr = font.chars[index];
                self.add_image(
                    font.texture, chr.x as i32, chr.y as i32, font.texture_w, font.texture_h,
                    x_at + chr.xoffset as i32, y + chr.yoffset as i32, chr.width as i32, chr.height as i32);
                x_at += chr.xadvance as i32;
                prev_char = chr.chr;
            }
        }
    }

    pub fn add_image(&mut self,
            source: &'static [u8], source_x: i32, source_y: i32, source_width: i32, _source_height: i32,
            dest_x: i32, dest_y: i32, width: i32, height: i32) {
        self.add_element(Box::new(Image {
            source: source,
            source_x: source_x,
            source_y: source_y,
            source_width: source_width,
            dest_x: dest_x,
            dest_y: dest_y,
            width: width,
            height: height,
        }));
    }

    pub fn add_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, thickness: i32) {
        let d = if thickness > 1 { thickness / 2  } else { 1 };
        let a = y2 - y1;
        let b = x1 - x2;
        let c = x2 * y1 - y2 * x1;
        let sqsum = a * a + b * b;
        self.add_element(Box::new(Line {
            x1: x1,
            y1: y1,
            x2: x2,
            y2: y2,
            d: d,
            a: a,
            b: b,
            c: c,
            dist_test: d * d * sqsum,
            minx_test: min(x1, x2) * sqsum + a * c,
            maxx_test: max(x1, x2) * sqsum + a * c,
            miny_test: min(y1, y2) * sqsum + b * c,
            maxy_test: max(y1, y2) * sqsum + b * c,
        }));
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

        self.x_starts.push(ElementEntry { value: bx + bw - 1, index: index });
        self.x_ends.push(ElementEntry { value: bx - 1, index: index });
        self.elements.push(renderable);
    }

    pub fn finish(&mut self) {
        self.x_starts.sort_by_key(|e| -e.value);
        self.x_ends.sort_by_key(|e| -e.value);
    }

    // Returns the pixels from right to left from top to bottom.
    pub fn render_pixel(&mut self) -> Color {
        if self.y_at == 0 {
            while self.x_starts_at < self.x_starts.len() && self.x_starts[self.x_starts_at].value == self.x_at {
                self.x_active.push(self.x_starts[self.x_starts_at].index);
                self.x_starts_at += 1;
            }
            while self.x_ends_at < self.x_ends.len() && self.x_ends[self.x_ends_at].value == self.x_at {
                self.x_active.remove_item(&self.x_ends[self.x_ends_at].index);
                self.x_ends_at += 1;
            }
        }

        let mut result = Color::TRANSPARENT;
        for &element_index in &self.x_active {
            if self.elements[element_index].render_pixel(self.x_at, self.y_at) == Color::BLACK {
                result = Color::BLACK;
                break;
            }
        }

        self.y_at += 1;
        if self.y_at == self.height {
            self.x_at -= 1;
            self.y_at = 0;
        }

        result
    }
}
