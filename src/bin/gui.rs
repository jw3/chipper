use clap::Clap;

use relm::Widget;

use libchip::widgets::{Win, ImageSource};
use libchip::args::GuiOpts;

fn main() {
    let opts = GuiOpts::parse();

    Win::run(ImageSource::File(opts.path)).expect("Win::run failed");
}
