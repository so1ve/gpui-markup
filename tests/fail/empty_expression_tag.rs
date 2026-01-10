use gpui_markup::ui;

fn main() {
    // Should fail: expression without braces at top level
    let _ = ui! { Button::new() };
}
