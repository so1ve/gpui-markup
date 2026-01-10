//! Dynamic content tests for gpui-markup.

use gpui::prelude::FluentBuilder;
use gpui::{InteractiveElement, ParentElement, StyleRefinement, Styled, div, px};
use gpui_markup::ui;

#[test]
fn test_dynamic_text() {
    let name = "Alice";
    let _ = ui! {
        <div>
            {format!("Hello, {}!", name)}
        </div>
    };
}

#[test]
fn test_dynamic_attribute_value() {
    let width = 300.0;
    let height = 200.0;
    let _ = ui! {
        <div w={px(width)} h={px(height)}/>
    };
}

#[test]
fn test_conditional_style_with_when() {
    let is_active = true;
    let _ = ui! {
        <div
            bg={gpui::black()}
            when={is_active, |s| s.border_color(gpui::blue())}
        />
    };
}

#[test]
fn test_hover_with_closure() {
    let _ = ui! {
        <div
            bg={gpui::white()}
            hover={|s: StyleRefinement| s.bg(gpui::black())}
        />
    };
}

#[test]
fn test_complex_expression() {
    let values = [10.0, 20.0, 30.0];
    let index = 1;
    let _ = ui! {
        <div w={px(values[index] * 2.0)}/>
    };
}

#[test]
fn test_method_chain_in_child() {
    let text = "hello world";
    let _ = ui! {
        <div>
            {text.to_uppercase().replace(' ', "_")}
        </div>
    };
}

#[test]
fn test_conditional_child() {
    let show_extra = true;
    let _ = ui! {
        <div>
            {if show_extra { "Extra content" } else { "" }}
        </div>
    };
}

#[test]
fn test_match_expression_in_child() {
    let status = 1;
    let _ = ui! {
        <div>
            {match status {
                0 => "Pending",
                1 => "Active",
                _ => "Unknown",
            }}
        </div>
    };
}

#[test]
fn test_block_expression_in_child() {
    let _ = ui! {
        <div>
            {{
                let x = 1 + 2;
                format!("Result: {x}")
            }}
        </div>
    };
}

#[test]
fn test_closure_call_in_child() {
    let make_text = || "Generated text";
    let _ = ui! {
        <div>
            {make_text()}
        </div>
    };
}

#[test]
fn test_option_unwrap_in_child() {
    let maybe_name: Option<&str> = Some("Alice");
    let _ = ui! {
        <div>
            {maybe_name.map(|n| format!("Hello, {n}")).unwrap_or_default()}
        </div>
    };
}

#[test]
fn test_conditional_attribute_value() {
    let level = 2;
    let _ = ui! {
        <div w={px(if level > 1 { 200.0 } else { 100.0 })}/>
    };
}

#[test]
fn test_struct_field_in_attribute() {
    struct Config {
        width: f32,
        height: f32,
    }
    let config = Config {
        width: 300.0,
        height: 200.0,
    };
    let _ = ui! {
        <div w={px(config.width)} h={px(config.height)}/>
    };
}

#[test]
fn test_multiple_children_mixed_types() {
    let data = ("Title", "Description");
    let _ = ui! {
        <div>
            {data.0}
            {data.1}
        </div>
    };
}

#[test]
fn test_array_index_in_child() {
    let items = ["First", "Second", "Third"];
    let _ = ui! {
        <div>
            {items[0]}
            {items[1]}
        </div>
    };
}

#[test]
fn test_deeply_nested_structure() {
    let _ = ui! {
        <div flex>
            <div flex_col>
                <div>
                    {"Level 1"}
                    <div>
                        {"Level 2"}
                        <div>
                            {"Level 3"}
                        </div>
                    </div>
                </div>
            </div>
        </div>
    };
}
