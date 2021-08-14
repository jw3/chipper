use gtk::{Window, Image, Inhibit, Builder};

use relm::{connect, Relm, Update, Widget, WidgetTest};
use relm_derive::Msg;
use gtk::prelude::{WidgetExt, BuilderExtManual, ImageExt};

pub struct Win {
    model: Model,
    widgets: Widgets,
}

pub enum ImageSource {
    File(String)
}

pub struct Model {
    source: ImageSource
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

        match &model.source {
            ImageSource::File(path) => {
                image_widget.set_from_file(path)
            }
        }

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
