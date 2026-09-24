use gpui::div;
use gpui_markup::ui;

struct Header;

impl Header {
    fn new() -> gpui::Div {
        div()
    }
}

fn main() {
    let _ = ui! {
        div
    };
    let _ = ui! {
        div {
            div
        }
    };
    let _ = ui! {
        div @[]
    };
    let _ = ui! {
        Header
    };
    let _ = ui! {
        div {
            Header
        }
    };
}
