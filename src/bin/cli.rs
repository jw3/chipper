use std::path::Path;

use clap::Clap;
use image::{GenericImage, GenericImageView};
use rayon::prelude::*;

use libchip::ImageType::Jpeg;
use libchip::Namer;

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

    let sz = opts.size;
    let namer = Namer::new(&opts.path, opts.outdir);

    let t = std::time::SystemTime::now();
    let source = image::open(&opts.path).expect("cant open");
    let (w, h) = source.dimensions();
    let (cw, ch) = (w / sz, h / sz);
    let (rw, rh) = ((0..cw), 0..ch);
    let rw: Vec<u32> = rw.collect();
    let rh: Vec<u32> = rh.collect();

    let v: Vec<(u32, u32)> = rw.iter().flat_map(|&ww| rh.iter().map(|&hh| (ww, hh)).collect::<Vec<(u32, u32)>>()).collect();
    v.par_iter().for_each(|(cx, cy)| {
        let xo = cx * sz;
        let yo = cy * sz;
        let name = namer.get(&format!("{}x{}", xo, yo), Jpeg);
        match source.clone().sub_image(xo, yo, sz, sz).to_image().save(name) {
            Ok(_) => print!("."),
            Err(_) => print!("x")
        };
    });
    let d = t.elapsed().unwrap();
    println!("{} chips in {} ms", v.len(), d.as_millis());
}
