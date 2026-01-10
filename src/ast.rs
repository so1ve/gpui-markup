//! AST definitions for gpui-markup DSL.

use syn::{Expr, Ident};

/// Root node of the markup DSL.
#[derive(Debug)]
pub struct Markup {
    pub element: Element,
}

/// An element in the markup tree.
#[derive(Debug)]
pub enum Element {
    /// Native elements: `<div>`, `<img>`, `<svg>`, `<canvas>`, `<anchored>`
    Native(NativeElement),
    /// `<Foo/>` - component without parameters
    Component(ComponentElement),
    /// `<{expr}/>` or `<{expr}>...</{}>` - expression as tag
    Expression(ExprElement),
}

/// Native GPUI elements (div, img, svg, canvas, anchored)
#[derive(Debug)]
pub struct NativeElement {
    pub open_name: Ident,
    pub close_name: Option<Ident>,
    pub attributes: Vec<Attribute>,
    pub children: Vec<Child>,
}

/// A component element (`PascalCase` identifier).
#[derive(Debug)]
pub struct ComponentElement {
    pub open_name: Ident,
    pub close_name: Option<Ident>,
    pub children: Vec<Child>,
}

/// An expression used as a tag.
#[derive(Debug)]
pub struct ExprElement {
    pub expr: Expr,
    pub attributes: Vec<Attribute>,
    pub children: Vec<Child>,
}

/// An attribute on an element.
#[derive(Debug)]
pub enum Attribute {
    /// Flag attribute: `flex`, `cursor_pointer`, etc.
    Flag(Ident),
    /// Key-value attribute: `w={px(200.0)}`, `when={cond, fn}`, etc.
    KeyValue { key: Ident, values: Vec<Expr> },
}

/// A child of an element.
#[derive(Debug)]
pub enum Child {
    /// A nested element
    Element(Element),
    /// An expression: `{expr}` or `{"text"}`
    Expression(Expr),
}
