use gpui_markup::ui;

fn main() {
    // Should fail: spread without expression
    let _ = ui! {
        <div>
            {..}
        </div>
    };
}
