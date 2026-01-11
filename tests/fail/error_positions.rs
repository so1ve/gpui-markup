use gpui::{deferred, div};
use gpui_markup::ui;

fn main() {
    // Should fail: `ParentElement` not implemented
    let _ = ui! {
        div {
            "" {
                ""
            }
        }
    };
    // Should fail: `IntoElement` not implemented
    let _ = ui! {
        deferred {
            1
        }
    };
}
