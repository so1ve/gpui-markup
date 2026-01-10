//! Basic syntax tests for gpui-markup.

use gpui::prelude::FluentBuilder;
use gpui::{FontWeight, InteractiveElement, IntoElement, ParentElement, Styled, deferred, div, px};
use gpui_markup::ui;

#[test]
fn test_empty_div() {
    let _ = ui! { <div/> };
}

#[test]
fn test_div_with_flag_attributes() {
    let _ = ui! { <div flex flex_col items_center/> };
}

#[test]
fn test_div_with_value_attributes() {
    let _ = ui! { <div w={px(200.0)} h={px(100.0)}/> };
}

#[test]
fn test_div_with_mixed_attributes() {
    let _ = ui! {
        <div flex w={px(200.0)} border_1/>
    };
}

#[test]
fn test_div_with_single_text_child() {
    let _ = ui! {
        <div>
            {"Hello World"}
        </div>
    };
}

#[test]
fn test_div_with_single_element_child() {
    let _ = ui! {
        <div>
            <div flex/>
        </div>
    };
}

#[test]
fn test_div_with_multiple_children() {
    let _ = ui! {
        <div>
            {"First"}
            {"Second"}
            {"Third"}
        </div>
    };
}

#[test]
fn test_nested_divs() {
    let _ = ui! {
        <div flex>
            <div flex_col>
                <div items_center>
                    {"Deeply nested"}
                </div>
            </div>
        </div>
    };
}

#[test]
fn test_hover_attribute() {
    let _ = ui! {
        <div hover={|s| s.bg(gpui::black())}/>
    };
}

#[test]
fn test_when_attribute() {
    let active = true;
    let _ = ui! {
        <div when={active, |s| s.border_color(gpui::blue())}/>
    };
}

#[test]
fn test_complex_styling() {
    let _ = ui! {
        <div
            flex
            flex_col
            w={px(200.0)}
            border_1
            rounded_md
            cursor_pointer
        />
    };
}

#[test]
fn test_text_styling() {
    let _ = ui! {
        <div
            text_size={px(16.0)}
            font_weight={FontWeight::BOLD}
        >
            {"Styled text"}
        </div>
    };
}

#[test]
fn test_expression_as_child() {
    let text = "Dynamic text".to_string();
    let _ = ui! {
        <div>
            {text}
        </div>
    };
}

#[test]
fn test_method_call_as_child() {
    let name = "World";
    let _ = ui! {
        <div>
            {format!("Hello, {}!", name)}
        </div>
    };
}

#[test]
fn test_deferred() {
    let element = div();
    let _ = ui! {
        <deferred>
            {element}
        </deferred>
    };
}
