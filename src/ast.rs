//! AST definitions for gpui-markup DSL.
//!
//! Optional child lists preserve the difference between an absent body and an
//! explicitly empty body, which code generation needs for editor completion.

use proc_macro2::TokenStream;
use syn::{Expr, Ident};

/// Root node of the markup DSL.
#[derive(Debug)]
pub struct Markup {
    pub element: Element,
    pub errors: Vec<syn::Error>,
}

/// An element in the markup tree.
#[derive(Debug)]
pub enum Element {
    /// Native elements: `div`, `svg`, `anchored`
    Native(NativeElement),
    /// Component elements: `Header`, `Button`, etc. (calls `::new()`
    /// implicitly)
    Component(ComponentElement),
    /// `deferred { child }`
    Deferred(DeferredElement),
    /// Expression as element: `(expr) [attrs] { children }`
    Expression(ExprElement),
}

#[derive(Debug)]
pub struct NativeElement {
    pub name: Ident,
    pub attributes: Vec<Attribute>,
    pub children: Option<Vec<Child>>,
}

#[derive(Debug)]
pub struct ComponentElement {
    pub name: Ident,
    pub attributes: Vec<Attribute>,
    pub children: Option<Vec<Child>>,
}

#[derive(Debug)]
pub struct DeferredElement {
    pub name: Ident,
    pub child: Box<Element>,
}

/// An expression used as an element.
#[derive(Debug)]
pub struct ExprElement {
    pub expr: Expr,
    pub attributes: Vec<Attribute>,
    pub children: Option<Vec<Child>>,
}

/// An attribute on an element.
#[derive(Debug)]
pub enum Attribute {
    /// Flag attribute: `flex`, `cursor_pointer`, etc.
    Flag(Ident),
    /// Key-value attribute: `w: px(200.0)`, `when: (cond, fn)`, etc.
    KeyValue { key: Ident, value: Expr },
}

/// A child of an element.
#[derive(Debug)]
pub enum Child {
    /// A nested element (including all expressions)
    Element(Element),
    /// A spread expression: `..expr` where expr is iterable
    Spread(Expr),
    /// A method chain: `.method(args)` or `.a().b::<T>()`
    MethodChain(TokenStream),
}
