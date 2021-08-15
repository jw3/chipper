use gtk::gdk::EventKey;
use relm::{connect, Relm, Update, Widget, WidgetTest};
use relm_derive::Msg;

#[derive(Msg)]
pub enum Msg {
    InputEvent(EventKey),
    Quit,
}
