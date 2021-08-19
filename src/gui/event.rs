use gtk::gdk::EventKey;

use relm_derive::Msg;

#[derive(Msg)]
pub enum Msg {
    InputEvent(EventKey),
    Quit,
}
