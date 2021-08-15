use clap::Clap;


use libchip::args::GuiOpts;
use libchip::widgets::app::App;

use libchip::{load_tif_image};
use relm::Widget;

fn main() {
    let opts = GuiOpts::parse();

    let full_image = load_tif_image(&opts.path, opts.mem).unwrap();
    App::run((full_image, opts.size)).expect("App::run failed");
}
