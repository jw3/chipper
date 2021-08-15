use clap::Clap;

use relm::Widget;

use libchip::widgets::{Win, Model, Cells};
use libchip::args::GuiOpts;
use libchip::{load_tif_image, Buffer};
use image::{GenericImage, GenericImageView};

fn main() {
    let opts = GuiOpts::parse();

    let mut full_image = load_tif_image(&opts.path ,opts.mem).unwrap();
    let (w, h) = full_image.dimensions();

    Win::run(Model{
        full_image,
        chip_size: opts.size,
        coords: (0, 0),
        bounds: Cells {
            w: w / opts.size,
            h: h / opts.size,
            wr: w % opts.size,
            hr: h % opts.size,
        }
    }).expect("Win::run failed");
}
