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

pub struct App {
    state: State,
    widgets: Widgets,
}

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
}

pub struct Cells {
    pub w: u32,
    pub h: u32,
    pub wr: u32,
    pub hr: u32,
}

impl State {
    fn chip(&mut self, id: (u32, u32), sz: (u32, u32)) -> Buffer {
        let (x, y, w, h) = (id.0 * sz.0, id.1 * sz.1, sz.0, sz.1);
        let bytes = self.full_image.sub_image(x, y, w, h).to_image().into_raw();
        Buffer {
            w,
            h,
            bytes,
        }
    }
    fn up(&mut self) -> Option<(u32, u32, u32, u32)> {
        if self.coords.1 > 0 {
            self.coords = (self.coords.0, self.coords.1 - 1);
            Some((self.coords.0, self.coords.1, self.chip_size, self.chip_size))
        } else {
            None
        }
    }
    fn down(&mut self) -> Option<(u32, u32, u32, u32)> {
        if self.coords.1 < self.bounds.h - 1 {
            self.coords = (self.coords.0, self.coords.1 + 1);
            Some((self.coords.0, self.coords.1, self.chip_size, self.chip_size))
        } else {
            None
        }
    }
    fn left(&mut self) -> Option<(u32, u32, u32, u32)> {
        if self.coords.0 > 0 {
            self.coords = (self.coords.0 - 1, self.coords.1);
            Some((self.coords.0, self.coords.1, self.chip_size, self.chip_size))
        } else {
            None
        }
    }
    fn right(&mut self) -> Option<(u32, u32, u32, u32)> {
        if self.coords.0 < self.bounds.w - 1 {
            self.coords = (self.coords.0 + 1, self.coords.1);
            Some((self.coords.0, self.coords.1, self.chip_size, self.chip_size))
        } else {
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

impl Update for App {
    type Model = State;
    type ModelParam = (DynamicImage, u32);
    type Msg = Msg;

    fn model(_: &Relm<Self>, param: (DynamicImage, u32)) -> State {
        State::new(param.0, param.1)
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::InputEvent(e) => {
                if let Some(letter) = e.keyval().to_unicode() {
                    if let Some((x, y, w, h)) = match letter {
                        'w' => self.state.up(),
                        's' => self.state.down(),
                        'a' => self.state.left(),
                        'd' => self.state.right(),
                        _ => None,
                    } {
                        let chip = self.state.chip((x, y), (w, h));
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

impl Widget for App {
    type Root = Window;

    fn init_view(&mut self) {
        let chip = self.state.chip((0, 0), (self.state.chip_size, self.state.chip_size));
        let b = Bytes::from_owned(chip.bytes);
        let pb = Pixbuf::from_bytes(&b, Colorspace::Rgb, true, 8, chip.w as i32, chip.h as i32, chip.w as i32 * 4);
        self.widgets.image_widget.set_from_pixbuf(Some(&pb));
    }

    fn root(&self) -> Self::Root {
        self.widgets.main_window.clone()
    }

    fn view(relm: &Relm<Self>, state: Self::Model) -> Self {
        let glade_src = include_str!("resources/chipper.glade");
        let builder = Builder::from_string(glade_src);

        let main_window: Window = builder.object("main_window").unwrap();
        connect!(relm, main_window, connect_delete_event(_, _), return (Some(Msg::Quit), Inhibit(false)));
        connect!(relm, main_window, connect_key_press_event(_, e), return (Some(Msg::InputEvent(e.clone())), Inhibit(false)));

        let image_widget: Image = builder.object("image_widget").unwrap();

        main_window.show_all();

        App {
            state,
            widgets: Widgets {
                image_widget,
                main_window,
            },
        }
    }
}
