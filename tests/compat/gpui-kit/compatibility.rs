// Run the same suite against Kit's GPUI without a separate GPUI dependency.
extern crate gpui_kit as gpui;

#[path = "../../basic.rs"]
mod basic;
#[path = "../../components.rs"]
mod components;
#[path = "../../dynamic.rs"]
mod dynamic;

mod kit {
    use gpui_kit::component::button::{Button, ButtonVariants};
    use gpui_kit::{self as gpui, Context, IntoElement, Render, Styled, Window, deferred, div};
    use gpui_markup::ui;

    struct KitView;

    impl Render for KitView {
        fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            ui! {
                div @[flex, flex_col, gap_2] {
                    Button::new("save") @[
                        primary,
                        label: "Save",
                        on_click: cx.listener(|_, _, _, cx| cx.notify()),
                    ] {},
                }
            }
        }
    }

    #[test]
    fn kit_components_in_markup() {
        let buttons = ["first", "second"].map(|id| {
            ui! { Button::new(id) @[label: id] {} }
        });
        let _: gpui::Div = ui! {
            div @[flex, flex_col] {
                Button::new("save") @[primary, label: "Save"] {},
                ..buttons,
                deferred {
                    div { Button::new("deferred") @[label: "Later"] {} },
                },
            }
        };
        let _ = KitView;
    }
}
