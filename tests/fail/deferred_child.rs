use gpui_markup::ui;

fn main() {
    // Should fail: deferred child is missing
    let _ = ui! { deferred {} };
    // Should fail: deferred with multiple children
    let _ = ui! {
        deferred {
            "1",
            "2",
        }
    };
}
