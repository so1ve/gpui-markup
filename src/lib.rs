//! gpui-markup - A declarative markup DSL for building GPUI applications.
//!
//! This crate provides a Rust-native syntax for building GPUI UIs:
//!
//! ```ignore
//! ui! {
//!     div @[flex, flex_col, w: px(200.0), bg: theme.secondary] {
//!         div @[text_size: px(16.0)] {
//!             "Hello World",
//!         },
//!     }
//! }
//! ```
//!
//! Which expands to:
//!
//! ```ignore
//! gpui::ParentElement::child(
//!     div()
//!         .flex()
//!         .flex_col()
//!         .w(px(200.0))
//!         .bg(theme.secondary),
//!     gpui::ParentElement::child(
//!         div().text_size(px(16.0)),
//!         "Hello World"
//!     )
//! )
//! ```

mod ast;
mod codegen;
mod parser;

use proc_macro::TokenStream;
use proc_macro_error2::proc_macro_error;
use quote::quote;
use syn::parse_macro_input;

use crate::ast::Markup;

/// A declarative markup macro for building GPUI UIs.
///
/// # Syntax
///
/// ## Basic Elements
///
/// ```ignore
/// ui! { div {} }                        // -> div()
/// ui! { div @[flex] {} }                // -> div().flex()
/// ui! { div @[w: px(200.0)] {} }        // -> div().w(px(200.0))
/// ```
///
/// ## Children
///
/// ```ignore
/// // Comma-separated children
/// ui! {
///     div {
///         "First",
///         "Second",
///     }
/// }
/// // -> gpui::ParentElement::child(gpui::ParentElement::child(div(), "First"), "Second")
/// ```
///
/// ## Spread Children
///
/// Use `..expr` to spread an iterable as children:
///
/// ```ignore
/// let items: Vec<Div> = vec![div(), div()];
///
/// ui! {
///     div {
///         ..items,
///     }
/// }
/// // -> gpui::ParentElement::children(div(), items)
///
/// // Can be mixed with regular children
/// ui! {
///     div {
///         "Header",
///         ..items,
///         "Footer",
///     }
/// }
/// // -> gpui::ParentElement::child(
/// //      gpui::ParentElement::children(
/// //        gpui::ParentElement::child(div(), "Header"),
/// //        items
/// //      ),
/// //      "Footer"
/// //    )
/// ```
///
/// ## Method Chains
///
/// Use `.method(args)` to insert method calls at any position.
/// Supports method chains and generics:
///
/// ```ignore
/// ui! {
///     div {
///         "static child",
///         .when(condition, |d| d.child("dynamic")),
///         .flex().gap_2(),
///         .map::<Div, _>(|d| d),
///     }
/// }
/// ```
///
/// ## Comments
///
/// Use standard Rust comments (`//` or `/* */`) inside `ui!`.
///
/// ## Expression Elements
///
/// Any expression can be used as an element (braces required at top level):
///
/// ```ignore
/// ui! { Button::new("Click") {} }              // -> Button::new("Click")
/// ui! { Button::new("Click") @[style: Primary] {} }
///                                              // -> Button::new("Click").style(Primary)
/// ui! {
///     div().flex() @[flex_col] {
///         "Content",
///     }
/// }
/// // -> gpui::ParentElement::child(div().flex().flex_col(), "Content")
///
/// // Parentheses for complex expressions (braces optional)
/// ui! { (a + b) }                              // -> a + b
/// ```
///
/// **Why braces at top level?** The `ui!` macro builds a component tree.
/// Braces declare "this is a UI element" - they mark it as a tree node,
/// trigger implicit `::new()` for components, and provide a place for
/// attributes and children.
///
/// ## Multi-value Attributes
///
/// Use tuples for attributes with multiple arguments:
///
/// ```ignore
/// ui! { div @[when: (condition, |d| d.flex())] {} }
/// // -> div().when(condition, |d| d.flex())
/// ```
#[proc_macro]
#[proc_macro_error]
pub fn ui(input: TokenStream) -> TokenStream {
    let markup = parse_macro_input!(input as Markup);
    let output = quote! { #markup };
    output.into()
}
