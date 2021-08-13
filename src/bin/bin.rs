use std::str::FromStr;

use clap::Clap;
use image::{GenericImage, GenericImageView};
use rayon::prelude::*;

use libchip::{BBox, ImageType, matrix, Namer, open_tif};
use libchip::cli::Opts;

fn main() {
    let opts: Opts = Opts::parse();
    let outfmt = ImageType::from_str(&opts.format).expect("Invalid image format");

    let sz = opts.size;

    let t = std::time::SystemTime::now();
    let source = open_tif(&opts.path, opts.mem).unwrap();
    let (w, h) = source.dimensions();
    println!("{} x {}", w, h);

    let v: Vec<BBox> = matrix((w, h), sz);
    let namer = Namer::new(&opts.path, opts.outdir);

    v.par_iter().for_each(|b| {
        let name = namer.make(&key(b.x, b.y), outfmt);
        match source.clone().sub_image(b.x, b.y, b.w, b.h).to_image().save(name) {
            Ok(_) => print!("."),
            Err(_) => print!("x")
        };
    });
    let d = t.elapsed().unwrap();
    println!("{} chips in {} ms", v.len(), d.as_millis());
}

fn key(x: u32, y: u32) -> String {
    format!("{}x{}", x, y)
}
