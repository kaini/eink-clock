use std::env;
use std::process::Command;
use std::fs;

fn svg_to_png(src: &str, dst: &str) {
    println!("cargo:rerun-if-changed={}", src);
    Command::new("inkscape").arg("-z")
                            .arg("-e").arg(dst)
                            .arg(src)
                            .status().unwrap();
}

fn image_to_gray(src: &str, dst: &str) {
    println!("cargo:rerun-if-changed={}", src);
    Command::new("magick").arg(src)
                          .arg("-depth").arg("1")
                          .arg(dst)
                          .status().unwrap();
}

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    
    // SVG Images
    svg_to_png("../graphics/clock.svg", &format!("{}/clock.png", out_dir));
    image_to_gray(&format!("{}/clock.png", out_dir), &format!("{}/clock.gray", out_dir));

    // Fonts
    image_to_gray("../graphics/large_0.png", &format!("{}/large.gray", out_dir));
    fs::copy("../graphics/large.fnt", format!("{}/large.fnt", out_dir)).unwrap();
    image_to_gray("../graphics/small_0.png", &format!("{}/small.gray", out_dir));
    fs::copy("../graphics/small.fnt", format!("{}/small.fnt", out_dir)).unwrap();
}
