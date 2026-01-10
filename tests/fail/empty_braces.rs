use gpui_markup::ui;

fn main() {
    // Should fail: empty braces are not allowed
    let _ = ui! {
        <div>
            {}
        </div>
    };
}
