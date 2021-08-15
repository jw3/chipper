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

pub struct Win {
    model: Model,
    widgets: Widgets,
}


pub struct Model {
    pub full_image: DynamicImage,
    pub chip_size: u32,
    pub coords: (u32, u32),
    pub bounds: Cells,
}

pub struct Cells {
    pub w: u32,
    pub h: u32,
    pub wr: u32,
    pub hr: u32,
}

impl Model {
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
        }
        else {
            None
        }
    }
    pub fn down(&mut self) -> Option<(u32, u32, u32, u32)> {
        if self.coords.1 < self.bounds.h - 1 {
            self.coords = (self.coords.0, self.coords.1 + 1);
            Some((self.coords.0, self.coords.1, self.chip_size, self.chip_size))
        }
        else {
            None
        }
    }
    pub fn left(&mut self) -> Option<(u32, u32, u32, u32)> {
        if self.coords.0 > 0 {
            self.coords = (self.coords.0 - 1, self.coords.1);
            Some((self.coords.0, self.coords.1, self.chip_size, self.chip_size))
        }
        else {
            None
        }
    }
    pub fn right(&mut self) -> Option<(u32, u32, u32, u32)> {
        if self.coords.0 < self.bounds.w - 1 {
            self.coords = (self.coords.0 + 1, self.coords.1);
            Some((self.coords.0, self.coords.1, self.chip_size, self.chip_size))
        }
        else {
            None
        }
    }
}

#[derive(Msg)]
pub enum Msg {
    InputEvent(EventKey),
    Quit,
}

#[derive(Clone)]
struct Widgets {
    image_widget: Image,
    main_window: Window,
}

impl Update for Win {
    type Model = Model;
    type ModelParam = Model;
    type Msg = Msg;

    fn model(_: &Relm<Self>, model: Model) -> Model {
        model
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::InputEvent(e) => {
                if let Some(letter) = e.keyval().to_unicode() {
                    if let Some((x, y, w, h)) = match letter {
                        'w' => self.model.up(),
                        's' => self.model.down(),
                        'a' => self.model.left(),
                        'd' => self.model.right(),
                        _ => None,
                    } {
                        let chip = self.model.chip((x, y), (w, h));
                        let b = Bytes::from_owned(chip.bytes);
                        let pb = Pixbuf::from_bytes(&b, Colorspace::Rgb, true, 8, chip.w as i32, chip.h as i32, chip.w as i32 * 4);
                        self.widgets.image_widget.set_from_pixbuf(Some(&pb));
                    }
                }
            }
            Msg::Quit => gtk::main_quit(),
        }
    }
}

impl Widget for Win {
    type Root = Window;

    fn root(&self) -> Self::Root {
        self.widgets.main_window.clone()
    }

    fn init_view(&mut self) {
        let chip = self.model.chip((0, 0), (self.model.chip_size, self.model.chip_size));
        let b = Bytes::from_owned(chip.bytes);
        let pb = Pixbuf::from_bytes(&b, Colorspace::Rgb, true, 8, chip.w as i32, chip.h as i32, chip.w as i32 * 4);
        self.widgets.image_widget.set_from_pixbuf(Some(&pb));
    }

    fn view(relm: &Relm<Self>, model: Self::Model) -> Self {
        let glade_src = include_str!("resources/chipper.glade");
        let builder = Builder::from_string(glade_src);

        let main_window: Window = builder.object("main_window").unwrap();
        connect!(relm, main_window, connect_delete_event(_, _), return (Some(Msg::Quit), Inhibit(false)));
        connect!(relm, main_window, connect_key_press_event(_, e), return (Some(Msg::InputEvent(e.clone())), Inhibit(false)));

        let image_widget: Image = builder.object("image_widget").unwrap();

        main_window.show_all();

        Win {
            model,
            widgets: Widgets {
                image_widget,
                main_window,
            },
        }
    }
}
