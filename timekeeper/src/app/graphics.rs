use collections::vec::Vec;
use alloc::boxed::Box;

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
            let source_bit = ((self.source_y + y) * self.source_width + self.source_x + x) as usize;
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
