use gtk::gdk::gdk_pixbuf::Colorspace;
use gtk::gdk_pixbuf::Pixbuf;
use gtk::glib::Bytes;
use gtk::prelude::{BuilderExtManual, ImageExt, WidgetExt};
use gtk::{Builder, Image, Inhibit, Window};
use image::DynamicImage;
use relm::{connect, Relm, Update, Widget};

use crate::widgets::event::Msg;
use crate::widgets::state::State;
use crate::widgets::Widgets;
use gtk::gdk::{EventKey, ModifierType};

pub struct App {
    state: State,
    widgets: Widgets,
}

fn with_ctrl(e: &EventKey) -> bool {
    e.state().contains(ModifierType::CONTROL_MASK)
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
                    if let Some(r) = match letter {
                        'w' => self.state.up(),
                        's' => self.state.down(),
                        'a' => self.state.left(),
                        'd' => self.state.right(),
                        'j' if with_ctrl(&e) => {
                            println!("save jpg");
                            None
                        }
                        'p' if with_ctrl(&e) => {
                            println!("save png");
                            None
                        }
                        _ => None,
                    } {
                        let chip = self.state.chip(r);
                        let b = Bytes::from_owned(chip.bytes);
                        let pb = Pixbuf::from_bytes(
                            &b,
                            Colorspace::Rgb,
                            true,
                            8,
                            chip.w as i32,
                            chip.h as i32,
                            chip.w as i32 * 4,
                        );
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
        let chip = self.state.chip(self.state.bounds);
        let b = Bytes::from_owned(chip.bytes);
        let pb = Pixbuf::from_bytes(
            &b,
            Colorspace::Rgb,
            true,
            8,
            chip.w as i32,
            chip.h as i32,
            chip.w as i32 * 4,
        );
        self.widgets.image_widget.set_from_pixbuf(Some(&pb));
    }

    fn root(&self) -> Self::Root {
        self.widgets.main_window.clone()
    }

    fn view(relm: &Relm<Self>, state: Self::Model) -> Self {
        let glade_src = include_str!("../resources/chipper.glade");
        let builder = Builder::from_string(glade_src);

        let main_window: Window = builder.object("main_window").unwrap();
        connect!(
            relm,
            main_window,
            connect_delete_event(_, _),
            return (Some(Msg::Quit), Inhibit(false))
        );
        connect!(
            relm,
            main_window,
            connect_key_press_event(_, e),
            return (Some(Msg::InputEvent(e.clone())), Inhibit(false))
        );

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
