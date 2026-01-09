//! Component tests for gpui-markup.

use gpui::{div, px, App, IntoElement, ParentElement, RenderOnce, Styled, Window};
use gpui_markup::ui;

/// A simple header component.
#[derive(IntoElement)]
struct Header;

impl Header {
    fn new() -> Self {
        Self
    }
}

impl RenderOnce for Header {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div().h(px(60.0)).w_full()
    }
}

/// A simple footer component.
#[derive(IntoElement)]
struct Footer;

impl Footer {
    fn new() -> Self {
        Self
    }
}

impl RenderOnce for Footer {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div().h(px(40.0)).w_full()
    }
}

#[test]
fn test_simple_component() {
    let _ = ui! { <Header/> };
}

#[test]
fn test_multiple_components() {
    let _ = ui! {
        <div>
            <Header/>
            <Footer/>
        </div>
    };
}

#[test]
fn test_expression_tag_with_children() {
    let _ = ui! {
        <{div().flex()} flex_col>
            {"Child content"}
        </{}>
    };
}

#[test]
fn test_mixed_components_and_divs() {
    let _ = ui! {
        <div flex>
            <Header/>
            <div flex_col>
                {"Content"}
            </div>
            <Footer/>
        </div>
    };
}

#[test]
fn test_nested_expression_tags() {
    let _ = ui! {
        <{div().p(px(16.0))} flex>
            <{div().rounded_md()} flex_col>
                {"Nested content"}
            </{}>
        </{}>
    };
}
