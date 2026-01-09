use gpui_markup::ui;

fn main() {
    // Should fail: mismatched closing tag
    let _ = ui! {
        <div>
        </span>
    };
}
