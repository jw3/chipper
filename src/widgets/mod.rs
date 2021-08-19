use gtk::{Image, Window};

pub mod app;
pub mod event;
pub mod state;

#[derive(Copy, Clone)]
pub struct Rect {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

impl Default for Rect {
    fn default() -> Self {
        Rect {
            x: 0,
            y: 0,
            w: 0,
            h: 0,
        }
    }
}

impl From<(u32, u32, u32, u32)> for Rect {
    fn from(v: (u32, u32, u32, u32)) -> Self {
        Rect {
            x: v.0,
            y: v.1,
            w: v.2,
            h: v.3,
        }
    }
}

pub struct Grid {
    pub w: u32,
    pub h: u32,
    pub wr: u32,
    pub hr: u32,
}

struct Widgets {
    image_widget: Image,
    main_window: Window,
}
