use image::{DynamicImage, GenericImage, GenericImageView};

use crate::widgets::{Grid, Rect};
use crate::Buffer;

pub struct State {
    pub full_image: DynamicImage,
    pub chip_size: u32,
    pub bounds: Rect,
    pub grid: Grid,
    pos: (u32, u32),
}

impl State {
    pub fn new(full_image: DynamicImage, chip_size: u32) -> Self {
        let (w, h) = full_image.dimensions();
        State {
            full_image,
            chip_size,
            bounds: Rect {
                w: chip_size,
                h: chip_size,
                ..Rect::default()
            },
            grid: Grid {
                w: w / chip_size,
                h: h / chip_size,
                wr: w % chip_size - 1,
                hr: h % chip_size - 1,
            },
            pos: (0, 0),
        }
    }

    pub fn chip(&mut self, r: Rect) -> Buffer {
        let bytes = self
            .full_image
            .sub_image(r.x, r.y, r.w, r.h)
            .to_image()
            .into_raw();
        Buffer {
            w: r.w,
            h: r.h,
            bytes,
        }
    }
    pub fn up(&mut self) -> Option<Rect> {
        if self.pos.1 > 0 {
            self.pos = (self.pos.0, self.pos.1 - 1);
            self.bounds = Rect {
                y: self.pos.1 * self.chip_size,
                ..self.bounds
            };
            Some(self.bounds)
        } else {
            None
        }
    }
    pub fn down(&mut self) -> Option<(Rect)> {
        if self.pos.1 + 1 < self.grid.h {
            self.pos = (self.pos.0, self.pos.1 + 1);
            self.bounds = Rect {
                y: self.pos.1 * self.chip_size,
                ..self.bounds
            };
            Some(self.bounds)
        } else {
            None
        }
    }
    pub fn left(&mut self) -> Option<(Rect)> {
        if self.pos.0 > 0 {
            self.pos = (self.pos.0 - 1, self.pos.1);
            self.bounds = Rect {
                x: self.pos.0 * self.chip_size,
                ..self.bounds
            };
            Some((self.bounds))
        } else {
            None
        }
    }
    pub fn right(&mut self) -> Option<(Rect)> {
        if self.pos.0 + 1 < self.grid.w {
            self.pos = (self.pos.0 + 1, self.pos.1);
            self.bounds = Rect {
                x: self.pos.0 * self.chip_size,
                ..self.bounds
            };
            Some((self.bounds))
        } else {
            None
        }
    }
}
