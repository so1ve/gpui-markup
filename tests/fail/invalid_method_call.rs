use gpui_markup::ui;

fn main() {
    // Should fail: method call without parentheses
    let _ = ui! {
        <div>
            {.flex}
        </div>
    };
}
