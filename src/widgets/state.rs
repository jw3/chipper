use image::{DynamicImage, GenericImage, GenericImageView};

use crate::widgets::{Grid, Rect};
use crate::Buffer;

pub struct State {
    pub full_image: DynamicImage,
    pub chip_size: u32,
    pub bounds: Rect,
    pub grid: Grid,
}

impl State {
    pub fn new(full_image: DynamicImage, chip_size: u32) -> Self {
        let (w, h) = full_image.dimensions();
        State {
            full_image,
            chip_size,
            bounds: Rect::default(),
            grid: Grid {
                w: w / chip_size,
                h: h / chip_size,
                wr: w % chip_size,
                hr: h % chip_size,
            },
        }
    }

    pub fn chip(&mut self, id: (u32, u32), sz: (u32, u32)) -> Buffer {
        let (x, y, w, h) = (id.0 * sz.0, id.1 * sz.1, sz.0, sz.1);
        let bytes = self.full_image.sub_image(x, y, w, h).to_image().into_raw();
        Buffer { w, h, bytes }
    }
    pub fn up(&mut self) -> Option<(u32, u32, u32, u32)> {
        if self.bounds.y > 0 {
            self.bounds = Rect {
                y: self.bounds.y - 1,
                ..self.bounds
            };
            Some((self.bounds.0, self.bounds.1, self.chip_size, self.chip_size))
        } else {
            None
        }
    }
    pub fn down(&mut self) -> Option<(u32, u32, u32, u32)> {
        if self.bounds.1 < self.grid.h - 1 {
            self.bounds = (self.bounds.0, self.bounds.1 + 1);
            Some((self.bounds.0, self.bounds.1, self.chip_size, self.chip_size))
        } else {
            None
        }
    }
    pub fn left(&mut self) -> Option<(u32, u32, u32, u32)> {
        if self.bounds.0 > 0 {
            self.bounds = (self.bounds.0 - 1, self.bounds.1);
            Some((self.bounds.0, self.bounds.1, self.chip_size, self.chip_size))
        } else {
            None
        }
    }
    pub fn right(&mut self) -> Option<(u32, u32, u32, u32)> {
        if self.bounds.0 < self.grid.w - 1 {
            self.bounds = (self.bounds.0 + 1, self.bounds.1);
            Some((self.bounds.0, self.bounds.1, self.chip_size, self.chip_size))
        } else {
            None
        }
    }
}
