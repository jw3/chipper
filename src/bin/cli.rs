use std::path::Path;

use clap::Clap;
use image::{GenericImage, GenericImageView};
use rayon::borrow::BorrowMut;

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
    let source_file = Path::new(&opts.path);
    let output_path = opts.outdir.unwrap_or("./".into());
    let output_fmt = opts.format;
    let base_name: String = source_file.file_name().unwrap().to_str().unwrap().into();
    let source_ext = source_file.extension().unwrap().to_str().unwrap() ;
    let base_name = base_name.strip_suffix(&format!(".{}", source_ext)).unwrap();

    let t = std::time::SystemTime::now();
    let source = image::open(&source_file).expect("cant open");
    let (w, h) = source.dimensions();
    let (cw, ch) = (w / sz, h / sz);
    let (rw, rh) = ((0..cw), 0..ch);
    let rw: Vec<u32> = rw.collect();
    let rh: Vec<u32> = rh.collect();

    let v: Vec<(u32, u32)> = rw.iter().flat_map(|&ww| rh.iter().map(|&hh| (ww, hh)).collect::<Vec<(u32, u32)>>()).collect();
    v.par_iter().for_each(|(cx, cy)| {
        let xo = cx * sz;
        let yo = cy * sz;
        let output_path = format!("{}/{}-{}x{}.{}", output_path, base_name, xo, yo, &output_fmt);
        match source.clone().sub_image(xo, yo, sz, sz).to_image().save(output_path) {
            Ok(_) => print!("."),
            Err(_) => print!("x")
        };
    });
    let d = t.elapsed().unwrap();
    println!("{} chips in {} ms", v.len(), d.as_millis());
}
