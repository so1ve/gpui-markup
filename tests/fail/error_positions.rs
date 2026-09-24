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
    // Should fail at the child expression, not at a generated binding.
    let _ = ui! {
        div {
            1
        }
    };
}
