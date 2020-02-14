use std::{fs, io, time::Instant};

extern crate svgbob;

fn main() -> io::Result<()> {
    let art = include_str!("../test_data/simple.bob");
    let t1 = Instant::now();
    fs::create_dir_all("out")?;
    fs::write("out/simple.svg", svgbob::to_svg(art))?;
    println!("took {}ms", t1.elapsed().as_millis());
    Ok(())
}
