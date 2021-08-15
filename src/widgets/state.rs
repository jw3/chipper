use gtk::{Window, Image, Inhibit, Builder};

use relm::{connect, Relm, Update, Widget, WidgetTest};
use relm_derive::Msg;
use gtk::prelude::{WidgetExt, BuilderExtManual, ImageExt};
use gtk::gdk_pixbuf::Pixbuf;
use crate::{load_tif_buffer, Buffer, load_tif_image};
use gtk::gdk::gdk_pixbuf::Colorspace;
use gtk::glib::Bytes;
use image::{GenericImageView, FilterType, SubImage, GenericImage, DynamicImage};
use std::time::SystemTime;
use gtk::gdk::{EventButton, EventKey};
use crate::widgets::Cells;


pub struct State {
    pub full_image: DynamicImage,
    pub chip_size: u32,
    pub coords: (u32, u32),
    pub bounds: Cells,
}


impl State {
    pub fn new(full_image: DynamicImage, chip_size: u32) -> Self {
        let (w, h) = full_image.dimensions();
        State {
            full_image,
            chip_size,
            coords: (0, 0),
            bounds: Cells {
                w: w / chip_size,
                h: h / chip_size,
                wr: w % chip_size,
                hr: h % chip_size,
            }
        }
    }

    pub fn chip(&mut self, id: (u32, u32), sz: (u32, u32)) -> Buffer {
        let (x, y, w, h) = (id.0 * sz.0, id.1 * sz.1, sz.0, sz.1);
        let bytes = self.full_image.sub_image(x, y, w, h).to_image().into_raw();
        Buffer {
            w,
            h,
            bytes,
        }
    }
    pub fn up(&mut self) -> Option<(u32, u32, u32, u32)> {
        if self.coords.1 > 0 {
            self.coords = (self.coords.0, self.coords.1 - 1);
            Some((self.coords.0, self.coords.1, self.chip_size, self.chip_size))
        } else {
            None
        }
    }
    pub fn down(&mut self) -> Option<(u32, u32, u32, u32)> {
        if self.coords.1 < self.bounds.h - 1 {
            self.coords = (self.coords.0, self.coords.1 + 1);
            Some((self.coords.0, self.coords.1, self.chip_size, self.chip_size))
        } else {
            None
        }
    }
    pub fn left(&mut self) -> Option<(u32, u32, u32, u32)> {
        if self.coords.0 > 0 {
            self.coords = (self.coords.0 - 1, self.coords.1);
            Some((self.coords.0, self.coords.1, self.chip_size, self.chip_size))
        } else {
            None
        }
    }
    pub fn right(&mut self) -> Option<(u32, u32, u32, u32)> {
        if self.coords.0 < self.bounds.w - 1 {
            self.coords = (self.coords.0 + 1, self.coords.1);
            Some((self.coords.0, self.coords.1, self.chip_size, self.chip_size))
        } else {
            None
        }
    }
}
