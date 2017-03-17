use collections::vec::Vec;
use alloc::boxed::Box;
use devices::flash::Font;

#[derive(Debug, PartialEq, Eq)]
pub enum Color {
    TRANSPARENT,
    BLACK,
}

pub trait Renderable {
    fn render_pixel(&self, x: i32, y: i32) -> Color;
}

pub struct Image {
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

impl Image {
    pub fn new(
            source: &'static [u8],
            source_x: i32,
            source_y: i32,
            source_width: i32,
            source_height: i32,
            dest_x: i32,
            dest_y: i32,
            width: i32,
            height: i32) -> Image {
        Image {
            source: source,
            source_x: source_x,
            source_y: source_y,
            source_width: source_width,
            source_height: source_height,
            dest_x: dest_x,
            dest_y: dest_y,
            width: width,
            height: height,
        }
    }
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
}

pub struct Text {
    images: Vec<Image>,  // Text drawn at (0, y)
    xoffset: i32,
}

pub enum HorizontalAlign {
    LEFT,
    CENTER,
    RIGHT,
}

impl Text {
    pub fn new(text: &str, font: &Font<'static>, x: i32, y: i32, halign: HorizontalAlign) -> Text {
        let mut images = Vec::new();
        let mut x_at = 0;
        let mut prev_char = 0;
        for c in text.chars() {
            if let Ok(index) = font.chars.binary_search_by_key(&(c as u32), |ref chr| chr.chr) {
                let kerning = if let Ok(index) = font.kerning_pairs.binary_search_by_key(&(prev_char, c as u32), |ref k| k.pair) {
                    font.kerning_pairs[index].amount as i32
                } else {
                    0
                };
                let ref chr = font.chars[index];
                images.push(Image::new(
                    font.texture, chr.x as i32, chr.y as i32, font.texture_w, font.texture_h,
                    x_at + chr.xoffset as i32 + kerning, y + chr.yoffset as i32, chr.width as i32, chr.height as i32));
                x_at += chr.xadvance as i32;
                prev_char = chr.chr;
            }
        }

        Text {
            images: images,
            xoffset: match halign {
                HorizontalAlign::LEFT => { x },
                HorizontalAlign::CENTER => { x - x_at / 2 },
                HorizontalAlign::RIGHT => { x - x_at }, 
            },
        }
    }
}

impl Renderable for Text {
    fn render_pixel(&self, x: i32, y: i32) -> Color {
        for image in self.images.iter() {
            if image.render_pixel(x - self.xoffset, y) == Color::BLACK {
                return Color::BLACK;
            }
        }
        Color::TRANSPARENT
    }
}

pub struct Graphic {
    elements: Vec<Box<Renderable>>,
}

impl Graphic {
    pub fn new() -> Graphic {
        Graphic {
            elements: Vec::new(),
        }
    }

    pub fn add_element(&mut self, renderable: Box<Renderable>) {
        self.elements.push(renderable);
    }

    pub fn render_pixel(&self, x: i32, y: i32) -> Color {
        for element in self.elements.iter() {
            if element.render_pixel(x, y) == Color::BLACK {
                return Color::BLACK;
            }
        }
        Color::TRANSPARENT
    }
}
