use clap::Clap;
use relm::Widget;

use libchip::args::GuiOpts;
use libchip::gui::app::App;
use libchip::load_tif_image;

fn main() {
    let opts = GuiOpts::parse();

    let full_image = load_tif_image(&opts.path, opts.mem).unwrap();
    App::run((full_image, opts.size)).expect("App::run failed");
}
