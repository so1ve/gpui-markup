//! gpui-markup - A declarative markup DSL for building GPUI applications.
//!
//! This crate provides a JSX-like syntax for building GPUI UIs:
//!
//! ```ignore
//! ui! {
//!     <div flex flex_col w={px(200.0)} bg={theme.secondary}>
//!         <div text_size={px(16.0)}>
//!             {"Hello World"}
//!         </div>
//!     </div>
//! }
//! ```
//!
//! Which expands to:
//!
//! ```ignore
//! div()
//!     .flex()
//!     .flex_col()
//!     .w(px(200.0))
//!     .bg(theme.secondary)
//!     .child(
//!         div()
//!             .text_size(px(16.0))
//!             .child("Hello World")
//!     )
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
/// ui! { <div/> }                    // -> div()
/// ui! { <div flex/> }               // -> div().flex()
/// ui! { <div w={px(200.0)}/> }      // -> div().w(px(200.0))
/// ```
///
/// ## Children
///
/// ```ignore
/// // Children use chained .child() calls
/// ui! {
///     <div>
///         {"First"}
///         {"Second"}
///     </div>
/// }
/// // -> div().child("First").child("Second")
/// ```
///
/// ## Spread Children
///
/// Use `{..expr}` to spread an iterable as children:
///
/// ```ignore
/// let items: Vec<Div> = vec![div(), div()];
///
/// ui! {
///     <div>
///         {..items}
///     </div>
/// }
/// // -> div().children(items)
///
/// // Can be mixed with regular children
/// ui! {
///     <div>
///         {"Header"}
///         {..items}
///         {"Footer"}
///     </div>
/// }
/// // -> div().child("Header").children(items).child("Footer")
/// ```
///
/// ## Method Chains
///
/// Use `{.method(args)}` to insert method calls at any position.
/// Supports method chains and generics:
///
/// ```ignore
/// ui! {
///     <div>
///         {"static child"}
///         {.when(condition, |d| d.child("dynamic"))}
///         {.flex().gap_2()}
///         {.map::<Div, _>(|d| d)}
///     </div>
/// }
/// ```
///
/// ## Comments
///
/// Use standard Rust comments (`//` or `/* */`) inside `ui!`.
///
/// ## Components
///
/// ```ignore
/// ui! { <Header/> }                 // -> Header::new()
/// ui! { <{NavItem::new(path)}/> }   // -> NavItem::new(path)
/// ```
///
/// ## Expression Tags
///
/// ```ignore
/// // Self-closing expression tag
/// ui! { <{Container::new(title)}/> }
///
/// // Expression tag with attributes and children
/// ui! {
///     <{Container::new(title)} flex>
///         {"Content"}
///     </{}>
/// }
/// ```
#[proc_macro]
#[proc_macro_error]
pub fn ui(input: TokenStream) -> TokenStream {
    let markup = parse_macro_input!(input as Markup);
    let output = quote! { #markup };
    output.into()
}
