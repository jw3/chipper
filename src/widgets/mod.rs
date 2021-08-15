
use gtk::{Image, Window};

pub mod app;
pub mod event;
pub mod state;

pub struct Cells {
    pub w: u32,
    pub h: u32,
    pub wr: u32,
    pub hr: u32,
}

struct Widgets {
    image_widget: Image,
    main_window: Window,
}
