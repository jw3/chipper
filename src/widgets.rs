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

pub struct Win {
    model: Model,
    widgets: Widgets,
    imgbuf: Option<Buffer>,
}

pub enum ImageSource {
    File(String),
}

pub struct Model {
    source: ImageSource
}

impl Model {
    // pub fn chip(&mut self, id: (u32, u32), sz: u32) -> DynamicImage {
    //     let (x, y) = (id.0 * sz,id.1 * sz);
    //     self.full_image.sub_image(x,y, sz, sz).
    // }
}

#[derive(Msg)]
pub enum Msg {
    Load,
    Quit,
}

#[derive(Clone)]
struct Widgets {
    image_widget: Image,
    main_window: Window,
}

impl Update for Win {
    type Model = Model;
    type ModelParam = ImageSource;
    type Msg = Msg;

    fn model(_: &Relm<Self>, source: ImageSource) -> Model {
        Model {
            source
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::Load => {
                self.widgets.image_widget = Image::from_pixbuf(None);
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

    fn view(relm: &Relm<Self>, model: Self::Model) -> Self {
        let glade_src = include_str!("resources/chipper.glade");
        let builder = Builder::from_string(glade_src);

        let main_window: Window = builder.object("main_window").unwrap();
        connect!(relm, main_window, connect_delete_event(_, _), return (Some(Msg::Quit), Inhibit(false)));

        let image_widget: Image = builder.object("image_widget").unwrap();

        let imgbuf=  match &model.source {
            ImageSource::File(path) => {
                let mut y = load_tif_image(path ,3).unwrap();
                let y = y.sub_image(0, 0, 544, 544);
                //let y = load_tif_buffer(path, 3).unwrap();
                let t = SystemTime::now();
                //let y = y.resize_exact(y.width()/8,y.height()/8,FilterType::Triangle);
                let d = t.elapsed().unwrap();
                println!("{}", d.as_millis());
                // let x = y.sub_image(0, 0, 544, 544);

                //let z = y.resize_exact(y.width()/4,y.height()/4, FilterType::Nearest);
                //z.save("/tmp/fooxxxxxxxxxxxx2.jpg").unwrap();
                let w = y.dimensions().0 as i32;
                let h = y.dimensions().1 as i32;
                let b = Bytes::from_owned(y.to_image().into_raw());
                let pb = Pixbuf::from_bytes(&b, Colorspace::Rgb, true, 8, w as i32, h as i32, w as i32*4);
                image_widget.set_from_pixbuf(Some(&pb));

                //image_widget.set_from_file("/tmp/fooxxxxxxxxxxxx2.jpg");


                None
            },
        };

        main_window.show_all();

        Win {
            model,
            widgets: Widgets {
                image_widget,
                main_window,
            },
            imgbuf,
        }
    }
}
