use gpui_markup::ui;

struct Button;

impl Button {
    fn new() -> gpui::Div {
        gpui::div()
    }
}

fn main() {
    // Should fail: expression without braces at top level
    let _ = ui! { Button::new() };
}
