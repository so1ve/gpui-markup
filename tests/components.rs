//! Component tests for gpui-markup.

use gpui::{App, IntoElement, ParentElement, RenderOnce, Styled, Window, div, px};
use gpui_markup::ui;

#[derive(IntoElement)]
struct Header {
    base: gpui::Div,
}

impl Header {
    fn new() -> Self {
        Self { base: div() }
    }
}

impl Styled for Header {
    fn style(&mut self) -> &mut gpui::StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for Header {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        self.base
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

/// Component as root element (calls `::new()` implicitly)
#[test]
fn test_simple_component() {
    let _ = ui! { Header {} };
}

/// Multiple components as children
#[test]
fn test_multiple_components() {
    let _ = ui! {
        div {
            Header {},
            Footer {},
        }
    };
}

/// Expression tag with children (explicit expression syntax)
#[test]
fn test_expression_tag_with_children() {
    let _ = ui! {
        (div().flex()) { [flex_col]
            "Child content",
        }
    };
}

/// Mixed components and native elements
#[test]
fn test_mixed_components_and_divs() {
    let _ = ui! {
        div { [flex]
            Header {},
            div { [flex_col]
                "Content",
            },
            Footer {},
        }
    };
}

/// Nested expression tags (for builder pattern usage)
#[test]
fn test_nested_expression_tags() {
    let _ = ui! {
        (div().p(px(16.0))) { [flex]
            (div().rounded_md()) { [flex_col]
                "Nested content",
            },
        }
    };
}

/// Component with children
#[test]
fn test_component_with_children() {
    let _ = ui! {
        Container {
            div { "Inside Container" },
        }
    };
}

/// Component with attributes (using custom method, not Styled trait)
#[test]
fn test_component_with_attributes() {
    let _ = ui! {
        Header { [flex] }
    };
}
