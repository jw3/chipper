use clap::Clap;
use image::{GenericImage, GenericImageView};
use rayon::prelude::*;

use libchip::{BBox, matrix, Namer, open_tif, ImageType};
use std::str::FromStr;

#[derive(Clap)]
#[clap(name = "Chipper")]
/// Image in, images out
struct Opts {
    /// Source image
    path: String,

    /// Chip size
    #[clap(short, long, default_value = "544")]
    size: u32,

    /// Chip format
    #[clap(long, default_value = "jpg")]
    format: String,

    /// Output directory
    #[clap(short, long)]
    outdir: Option<String>,
}

fn main() {
    let opts: Opts = Opts::parse();
    let outfmt = ImageType::from_str(&opts.format).expect("Invalid image format");

    let sz = opts.size;

    let t = std::time::SystemTime::now();
    let source = open_tif(&opts.path).unwrap();
    let (w, h) = source.dimensions();

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
