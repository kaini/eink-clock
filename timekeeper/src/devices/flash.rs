pub struct Font<'a> {
    pub chars: &'a [FontChar],
    pub kerning_pairs: &'a [KerningPair],
    pub base: u16,
    pub texture: &'a [u8],
    pub texture_w: i32,
    pub texture_h: i32,
}

pub struct FontChar {
    pub chr: u32,
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
    pub xoffset: i16,
    pub yoffset: i16,
    pub xadvance: i16,
}

pub struct KerningPair {
    pub pair: (u32, u32),
    pub amount: i16,
}

pub const CLOCK: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/clock.gray"));
pub const CLOCK_W: i32 = 600;
pub const CLOCK_H: i32 = 600;

mod large_font {
    use super::{Font, FontChar, KerningPair};
    const TEXTURE: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/large.gray"));
    const TEXTURE_W: i32 = 512;
    const TEXTURE_H: i32 = 350;
    include!(concat!(env!("OUT_DIR"), "/large_font.rs"));
}
pub use self::large_font::FONT as LARGE_FONT;

mod small_font {
    use super::{Font, FontChar, KerningPair};
    const TEXTURE: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small.gray"));
    const TEXTURE_W: i32 = 128;
    const TEXTURE_H: i32 = 128;
    include!(concat!(env!("OUT_DIR"), "/small_font.rs"));
}
pub use self::small_font::FONT as SMALL_FONT;
