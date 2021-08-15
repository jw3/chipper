use clap::Clap;

use relm::Widget;
use libchip::widgets::app::App;
use libchip::widgets::{ Cells};
use libchip::args::GuiOpts;
use libchip::{load_tif_image, Buffer};
use image::{GenericImage, GenericImageView};

fn main() {
    let opts = GuiOpts::parse();

    let mut full_image = load_tif_image(&opts.path, opts.mem).unwrap();
    App::run((full_image, opts.size)).expect("Win::run failed");
}
