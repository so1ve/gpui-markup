//! Parser for gpui-markup DSL.
//!
//! Syntax:
//! - `div { [flex, w: px(200.0)] "Content", child }` - native element with attrs
//! - `div { "Content" }` - no attributes
//! - `div {}` - minimal
//! - `deferred { child }` - deferred element
//! - `(Button::new("Hi")) { [style: Primary] }` - expression element (wrapped in
//!   parens)

use proc_macro_error2::abort;
use proc_macro2::TokenStream;
use syn::ext::IdentExt;
use syn::parse::{Parse, ParseStream};
use syn::token::{Brace, Bracket, Paren};
use syn::{Expr, Ident, Result, Token, braced, bracketed, parenthesized};

use crate::ast::{
    Attribute, Child, ComponentElement, DeferredElement, Element, ExprElement, Markup,
    NativeElement,
};

/// Native GPUI element names
const NATIVE_ELEMENTS: &[&str] = &["div", "svg", "anchored"];

impl Parse for Markup {
    fn parse(input: ParseStream) -> Result<Self> {
        let element = parse_element(input)?;
        Ok(Self { element })
    }
}

/// Parse an element at the top level or as a child.
fn parse_element(input: ParseStream) -> Result<Element> {
    if input.peek(Paren) {
        // (expr) [...] {...} - expression element
        parse_paren_expression_element(input)
    } else if input.peek(Ident::peek_any) {
        let ident = input.call(Ident::parse_any)?;
        let name = ident.to_string();

        if name == "deferred" {
            parse_deferred_element(input, ident)
        } else if NATIVE_ELEMENTS.contains(&name.as_str()) {
            parse_native_element(input, ident)
        } else {
            // Component element: Header, Button, etc.
            parse_component_element(input, ident)
        }
    } else {
        abort!(
            input.span(),
            "Expected element: native element (div, svg, etc.), component, or parenthesized expression ((expr))"
        );
    }
}

/// Parse a native element: `div { [attrs] children }`
fn parse_native_element(input: ParseStream, name: Ident) -> Result<Element> {
    let (attributes, children) = parse_required_children(input, &name)?;

    Ok(Element::Native(NativeElement {
        name,
        attributes,
        children,
    }))
}

/// Parse a component element: `Header { [attrs] children }`
/// Generates `Header::new()...`
fn parse_component_element(input: ParseStream, name: Ident) -> Result<Element> {
    let (attributes, children) = parse_required_children(input, &name)?;

    Ok(Element::Component(ComponentElement {
        name,
        attributes,
        children,
    }))
}

/// Parse deferred element: `deferred { child }`
fn parse_deferred_element(input: ParseStream, name: Ident) -> Result<Element> {
    if !input.peek(Brace) {
        abort!(
            name.span(),
            "deferred must have exactly one child: `deferred {{ child }}`"
        );
    }

    let content;
    braced!(content in input);

    let child = parse_child(&content)?;

    // Consume trailing comma if present
    if content.peek(Token![,]) {
        content.parse::<Token![,]>()?;
    }

    if !content.is_empty() {
        abort!(content.span(), "deferred must have exactly one child");
    }

    Ok(Element::Deferred(DeferredElement {
        name,
        child: Box::new(child),
    }))
}

/// Parse an expression element wrapped in parens: `(expr) { [attrs] children }`
fn parse_paren_expression_element(input: ParseStream) -> Result<Element> {
    let content;
    parenthesized!(content in input);
    let expr: Expr = content.parse()?;

    let (attributes, children) = parse_optional_children(input)?;

    Ok(Element::Expression(ExprElement {
        expr,
        attributes,
        children,
    }))
}

/// Parse optional attributes in `[...]`
fn parse_optional_attributes(input: ParseStream) -> Result<Vec<Attribute>> {
    if !input.peek(Bracket) {
        return Ok(vec![]);
    }

    let content;
    bracketed!(content in input);

    let mut attributes = Vec::new();

    while !content.is_empty() {
        let attr = parse_attribute(&content)?;
        attributes.push(attr);

        if content.peek(Token![,]) {
            content.parse::<Token![,]>()?;
        } else {
            break;
        }
    }

    Ok(attributes)
}

/// Parse a single attribute: `flex` or `w: px(200.0)`
fn parse_attribute(input: ParseStream) -> Result<Attribute> {
    let key = input.call(Ident::parse_any)?;

    if !input.peek(Token![:]) {
        return Ok(Attribute::Flag(key));
    }

    input.parse::<Token![:]>()?;
    let value: Expr = input.parse()?;

    Ok(Attribute::KeyValue { key, value })
}

/// Parse optional children in `{ [attrs] ... }`
fn parse_optional_children(input: ParseStream) -> Result<(Vec<Attribute>, Vec<Child>)> {
    if !input.peek(Brace) {
        return Ok((vec![], vec![]));
    }

    let content;
    braced!(content in input);

    let attributes = parse_optional_attributes(&content)?;
    let children = parse_children(&content)?;
    Ok((attributes, children))
}

/// Parse required children in `{ [attrs] ... }` - braces are mandatory
fn parse_required_children(input: ParseStream, name: &Ident) -> Result<(Vec<Attribute>, Vec<Child>)> {
    if !input.peek(Brace) {
        abort!(
            name.span(),
            "element `{}` requires braces: `{} {{}}`",
            name,
            name
        );
    }

    let content;
    braced!(content in input);

    let attributes = parse_optional_attributes(&content)?;
    let children = parse_children(&content)?;
    Ok((attributes, children))
}

/// Parse comma-separated children
fn parse_children(input: ParseStream) -> Result<Vec<Child>> {
    let mut children = Vec::new();

    while !input.is_empty() {
        let child = parse_child(input)?;
        children.push(child);

        if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
        } else {
            break;
        }
    }

    Ok(children)
}

/// Parse a single child
fn parse_child(input: ParseStream) -> Result<Child> {
    // Spread: `..expr`
    if input.peek(Token![..]) {
        input.parse::<Token![..]>()?;
        let expr: Expr = input.parse()?;
        return Ok(Child::Spread(expr));
    }

    // Method chain: `.method()`
    if input.peek(Token![.]) {
        input.parse::<Token![.]>()?;
        let tokens = parse_method_chain(input)?;
        return Ok(Child::MethodChain(tokens));
    }

    // Element: native, deferred, or component
    // With required braces, detection is cleaner:
    // - Native/deferred with {...} -> element (will error if braces missing)
    // - Component with {...} -> element
    // - identifier alone or identifier[expr] without {} -> expression
    if input.peek(Ident::peek_any) {
        let fork = input.fork();
        let ident = fork.call(Ident::parse_any)?;
        let name = ident.to_string();

        // Native/deferred: always parse as element (will error if braces missing)
        if name == "deferred" || NATIVE_ELEMENTS.contains(&name.as_str()) {
            let element = parse_element(input)?;
            return Ok(Child::Element(element));
        }

        // Component: check if it has braces
        if fork.peek(Brace) {
            // `Header {...}` - element
            let element = parse_element(input)?;
            return Ok(Child::Element(element));
        }
        // Otherwise fall through to expression (e.g., `items[0]`, `SomeIdent`)
    }

    // Parenthesized: could be expression element or just grouped expression
    if input.peek(Paren) {
        let content;
        parenthesized!(content in input);
        let expr: Expr = content.parse()?;

        // If followed by {...}, it's an expression element
        if input.peek(Brace) {
            let (attributes, children) = parse_optional_children(input)?;
            return Ok(Child::Element(Element::Expression(ExprElement {
                expr,
                attributes,
                children,
            })));
        }

        // Otherwise it's just a parenthesized expression
        return Ok(Child::Expression(Expr::Paren(syn::ExprParen {
            attrs: vec![],
            paren_token: syn::token::Paren::default(),
            expr: Box::new(expr),
        })));
    }

    // Simple expression
    let expr: Expr = input.parse()?;
    Ok(Child::Expression(expr))
}

/// Parse a method chain until comma.
///
/// We need to track angle bracket depth because `<>` are not paired delimiters
/// in Rust's tokenizer (unlike `()`, `[]`, `{}`). They are parsed as individual
/// `Punct` tokens, so commas inside generics like `.map::<Div, _>()` would
/// incorrectly terminate the method chain without this tracking.
fn parse_method_chain(input: ParseStream) -> Result<TokenStream> {
    let mut tokens = TokenStream::new();
    let mut angle_depth = 0i32;

    while !input.is_empty() {
        // Only stop on comma when not inside angle brackets (generics)
        if input.peek(Token![,]) && angle_depth == 0 {
            break;
        }

        let tt: proc_macro2::TokenTree = input.parse()?;

        // Track angle bracket depth for generics
        if let proc_macro2::TokenTree::Punct(p) = &tt {
            match p.as_char() {
                '<' => angle_depth += 1,
                '>' => angle_depth = (angle_depth - 1).max(0),
                _ => {}
            }
        }

        tokens.extend(std::iter::once(tt));
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_div() {
        let input: proc_macro2::TokenStream = quote::quote! { div {} };
        let markup: Markup = syn::parse2(input).unwrap();
        assert!(matches!(markup.element, Element::Native(_)));
    }

    #[test]
    fn test_parse_div_with_attributes() {
        let input: proc_macro2::TokenStream = quote::quote! {
            div { [flex, w: px(200.0)] }
        };
        let markup: Markup = syn::parse2(input).unwrap();
        if let Element::Native(el) = markup.element {
            assert_eq!(el.attributes.len(), 2);
        } else {
            panic!("Expected Native element");
        }
    }

    #[test]
    fn test_parse_div_with_children() {
        let input: proc_macro2::TokenStream = quote::quote! {
            div {
                "Hello",
            }
        };
        let markup: Markup = syn::parse2(input).unwrap();
        if let Element::Native(el) = markup.element {
            assert_eq!(el.children.len(), 1);
        } else {
            panic!("Expected Native element");
        }
    }

    #[test]
    fn test_parse_div_full() {
        let input: proc_macro2::TokenStream = quote::quote! {
            div { [flex, flex_col]
                "Content",
                div { [bold] "Nested" },
            }
        };
        let markup: Markup = syn::parse2(input).unwrap();
        if let Element::Native(el) = markup.element {
            assert_eq!(el.attributes.len(), 2);
            assert_eq!(el.children.len(), 2);
        } else {
            panic!("Expected Native element");
        }
    }

    #[test]
    fn test_parse_expression_element() {
        let input: proc_macro2::TokenStream = quote::quote! {
            (Container::new(title))
        };
        let markup: Markup = syn::parse2(input).unwrap();
        assert!(matches!(markup.element, Element::Expression(_)));
    }

    #[test]
    fn test_parse_expression_element_with_attrs() {
        let input: proc_macro2::TokenStream = quote::quote! {
            (Button::new("Click")) { [style: Primary] }
        };
        let markup: Markup = syn::parse2(input).unwrap();
        if let Element::Expression(el) = markup.element {
            assert_eq!(el.attributes.len(), 1);
        } else {
            panic!("Expected Expression element");
        }
    }

    #[test]
    fn test_parse_spread_children() {
        let input: proc_macro2::TokenStream = quote::quote! {
            div {
                ..items,
            }
        };
        let markup: Markup = syn::parse2(input).unwrap();
        if let Element::Native(el) = markup.element {
            assert_eq!(el.children.len(), 1);
            assert!(matches!(el.children[0], Child::Spread(_)));
        } else {
            panic!("Expected Native element");
        }
    }

    #[test]
    fn test_parse_method_chain() {
        let input: proc_macro2::TokenStream = quote::quote! {
            div {
                .when(cond, |d| d.flex()),
            }
        };
        let markup: Markup = syn::parse2(input).unwrap();
        if let Element::Native(el) = markup.element {
            assert_eq!(el.children.len(), 1);
            assert!(matches!(el.children[0], Child::MethodChain(_)));
        } else {
            panic!("Expected Native element");
        }
    }

    #[test]
    fn test_parse_deferred() {
        let input: proc_macro2::TokenStream = quote::quote! {
            deferred {
                div { "Content" },
            }
        };
        let markup: Markup = syn::parse2(input).unwrap();
        assert!(matches!(markup.element, Element::Deferred(_)));
    }

    #[test]
    fn test_parse_method_with_generics() {
        let input: proc_macro2::TokenStream = quote::quote! {
            div {
                .map::<Div, _>(|d| d),
            }
        };
        let markup: Markup = syn::parse2(input).unwrap();
        if let Element::Native(el) = markup.element {
            assert!(matches!(el.children[0], Child::MethodChain(_)));
        } else {
            panic!("Expected Native element");
        }
    }

    #[test]
    fn test_parse_paren_child_without_attrs() {
        let input: proc_macro2::TokenStream = quote::quote! {
            div {
                (some_expr),
            }
        };
        let markup: Markup = syn::parse2(input).unwrap();
        if let Element::Native(el) = markup.element {
            assert!(matches!(el.children[0], Child::Expression(_)));
        } else {
            panic!("Expected Native element");
        }
    }

    #[test]
    fn test_parse_paren_child_with_attrs() {
        let input: proc_macro2::TokenStream = quote::quote! {
            div {
                (Button::new()) { [flex] },
            }
        };
        let markup: Markup = syn::parse2(input).unwrap();
        if let Element::Native(el) = markup.element {
            assert!(matches!(
                el.children[0],
                Child::Element(Element::Expression(_))
            ));
        } else {
            panic!("Expected Native element");
        }
    }

    #[test]
    fn test_parse_component() {
        let input: proc_macro2::TokenStream = quote::quote! { Header {} };
        let markup: Markup = syn::parse2(input).unwrap();
        assert!(matches!(markup.element, Element::Component(_)));
    }

    #[test]
    fn test_parse_component_with_attrs() {
        let input: proc_macro2::TokenStream = quote::quote! {
            Header { [flex, style: Primary] }
        };
        let markup: Markup = syn::parse2(input).unwrap();
        if let Element::Component(el) = markup.element {
            assert_eq!(el.attributes.len(), 2);
        } else {
            panic!("Expected Component element");
        }
    }

    #[test]
    fn test_parse_component_with_children() {
        let input: proc_macro2::TokenStream = quote::quote! {
            Container {
                "Content",
                div { "Nested" },
            }
        };
        let markup: Markup = syn::parse2(input).unwrap();
        if let Element::Component(el) = markup.element {
            assert_eq!(el.children.len(), 2);
        } else {
            panic!("Expected Component element");
        }
    }

    #[test]
    fn test_parse_component_child() {
        let input: proc_macro2::TokenStream = quote::quote! {
            div {
                Header { [flex] },
            }
        };
        let markup: Markup = syn::parse2(input).unwrap();
        if let Element::Native(el) = markup.element {
            assert!(matches!(
                el.children[0],
                Child::Element(Element::Component(_))
            ));
        } else {
            panic!("Expected Native element");
        }
    }
}
