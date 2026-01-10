//! Component tests for gpui-markup.

use gpui::{App, IntoElement, ParentElement, RenderOnce, Styled, Window, div, px};
use gpui_markup::ui;

#[derive(IntoElement)]
struct Header;

impl Header {
    const fn new() -> Self {
        Self
    }
}

impl RenderOnce for Header {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
    }
}

#[derive(IntoElement)]
struct Footer;

impl Footer {
    const fn new() -> Self {
        Self
    }
}

impl RenderOnce for Footer {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
    }
}

#[derive(IntoElement)]
struct Container;

impl Container {
    const fn new() -> Self {
        Self
    }
}

impl ParentElement for Container {
    fn extend(&mut self, _elements: impl IntoIterator<Item = gpui::AnyElement>) {}
}

impl RenderOnce for Container {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
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

#[test]
fn test_component_with_children() {
    let _ = ui! {
        <Container>
            <div>{"Inside Container"}</div>
        </Container>
    };
}
