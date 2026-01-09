use gpui_markup::ui;

fn main() {
    // Should fail: components must be self-closing
    let _ = ui! {
        <Header>
        </Header>
    };
}
