use gpui_markup::ui;

fn main() {
    // Should fail: empty expression in tag
    let _ = ui! { <{}/> };
}
