use gpui_markup::ui;

fn main() {
    // Should fail: unknown element name
    let _ = ui! { <span/> };
}
