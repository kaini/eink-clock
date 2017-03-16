pub static CLOCK: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/clock.gray"));
pub const CLOCK_W: i32 = 600;
pub const CLOCK_H: i32 = 600;

pub static LARGE_LETTERS: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/large.gray"));
pub static LARGE_FONT: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/large.fnt"));

pub static SMALL_LETTERS: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small.gray"));
pub static SMALL_FONT: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small.fnt"));
