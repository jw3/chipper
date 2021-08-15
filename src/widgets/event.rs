
use relm::{connect, Relm, Update, Widget, WidgetTest};
use relm_derive::Msg;
use gtk::gdk::EventKey;


#[derive(Msg)]
pub enum Msg {
    InputEvent(EventKey),
    Quit,
}
