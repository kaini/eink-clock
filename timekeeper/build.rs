use std::env;
use std::process::Command;
use std::io::{Read, Write};
use std::fs::File;

fn parse_u16(data: &[u8]) -> u16 {
    data[0] as u16 | ((data[1] as u16) << 8)
}

fn parse_i16(data: &[u8]) -> i16 {
    parse_u16(data) as i16
}

fn parse_u32(data: &[u8]) -> u32 {
    data[0] as u32 | ((data[1] as u32) << 8) | ((data[2] as u32) << 16) | ((data[3] as u32) << 24)
}

fn svg_to_png(src: &str, dst: &str) {
    println!("cargo:rerun-if-changed={}", src);
    Command::new("inkscape").arg("-z")
                            .arg("-e").arg(dst)
                            .arg(src)
                            .status().unwrap();
}

fn image_to_gray(src: &str, dst: &str, invert: bool) {
    println!("cargo:rerun-if-changed={}", src);
    let mut cmd = Command::new("magick");
    cmd.arg(src)
       .arg("-depth").arg("1");
    if invert {
        cmd.arg("-negate");
    }
    cmd.arg(dst)
       .status().unwrap();
}

fn font_to_rust(src: &str, dst: &str) {
    // See http://www.angelcode.com/products/bmfont/doc/file_format.html#tags for the file format.
    
    println!("cargo:rerun-if-changed={}", src);

    let data = {
        let mut src_fp = File::open(src).unwrap();
        let mut data = Vec::new();
        src_fp.read_to_end(&mut data).unwrap();
        data
    };

    // Find the interesting blocks
    let mut chars_block = 0;
    let mut chars_size = 0;
    let mut common_block = 0;
    let mut kerning_block = 0;
    let mut kerning_size = 0;
    let mut at = 4;
    while at < data.len() {
        let this_block_size = parse_u32(&data[(at + 1)..(at + 5)]) as usize;
        let this_block_start = at + 5;
        match data[at] {
            2 => { common_block = this_block_start; },
            4 => { chars_block = this_block_start; chars_size = this_block_size; },
            5 => { kerning_block = this_block_start; kerning_size = this_block_size; },
            _ => {},
        }
        at = this_block_start + this_block_size;
    }

    let mut dst_fp = File::create(dst).unwrap();

    // Write the chars
    let mut lines = Vec::new();
    for i in 0..(chars_size / 20) {
        let base = chars_block + 20 * i;
        let line = format!("    FontChar {{ chr: {}, x: {}, y: {}, width: {}, height: {}, xoffset: {}, yoffset: {}, xadvance: {}, }}, // {:?}",
            parse_u32(&data[base..(base + 4)]),
            parse_u16(&data[(base + 4)..(base + 6)]),
            parse_u16(&data[(base + 6)..(base + 8)]),
            parse_u16(&data[(base + 8)..(base + 10)]),
            parse_u16(&data[(base + 10)..(base + 12)]),
            parse_i16(&data[(base + 12)..(base + 14)]),
            parse_i16(&data[(base + 14)..(base + 16)]),
            parse_i16(&data[(base + 16)..(base + 18)]),
            std::char::from_u32(parse_u32(&data[base..(base + 4)])).unwrap());
        lines.push((parse_u32(&data[base..(base + 4)]), line));
    }
    lines.sort_by_key(|&(chr, _)| chr);
    writeln!(dst_fp, "const CHARS: &[FontChar] = &[").unwrap();
    for &(_, ref line) in lines.iter() {
        writeln!(dst_fp, "{}", line).unwrap();
    }
    writeln!(dst_fp, "];").unwrap();

    // Write the kerning pairs
    let mut lines = Vec::new();
    for i in 0..(kerning_size / 10) {
        let base = kerning_block + 10 * i;
        let tuple = (
            std::char::from_u32(parse_u32(&data[base..(base + 4)])).unwrap(),
            std::char::from_u32(parse_u32(&data[(base + 4)..(base + 8)])).unwrap());
        let line = format!("    KerningPair {{ pair: ({}, {}), amount: {} }}, // {:?}",
            parse_u32(&data[base..(base + 4)]),
            parse_u32(&data[(base + 4)..(base + 8)]),
            parse_i16(&data[(base + 8)..(base + 10)]),
            tuple);
        lines.push((tuple, line));
    }
    lines.sort_by_key(|&(tuple, _)| tuple);
    writeln!(dst_fp, "const KERNING_PAIRS: &[KerningPair] = &[").unwrap();
    for &(_, ref line) in lines.iter() {
        writeln!(dst_fp, "{}", line).unwrap();
    }
    writeln!(dst_fp, "];").unwrap();

    // Write the font
    writeln!(dst_fp, "pub const FONT: Font = Font {{ chars: &CHARS, kerning_pairs: &KERNING_PAIRS, base: {}, texture: &TEXTURE, texture_w: TEXTURE_W, texture_h: TEXTURE_H, }};",
        parse_u16(&data[(common_block + 2)..(common_block + 4)])).unwrap();

    dst_fp.flush().unwrap();
}

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    
    // SVG Images
    svg_to_png("../graphics/clock.svg", &format!("{}/clock.png", out_dir));
    image_to_gray(&format!("{}/clock.png", out_dir), &format!("{}/clock.gray", out_dir), false);

    // Fonts
    image_to_gray("../graphics/large_0.png", &format!("{}/large.gray", out_dir), true);
    font_to_rust("../graphics/large.fnt", &format!("{}/large_font.rs", out_dir));
    image_to_gray("../graphics/small_0.png", &format!("{}/small.gray", out_dir), true);
    font_to_rust("../graphics/small.fnt", &format!("{}/small_font.rs", out_dir));
}
