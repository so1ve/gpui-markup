//! Parser for gpui-markup DSL.
//!
//! Syntax:
//! - `div @[flex, w: px(200.0)] { "Content", child }` - native element with
//!   attrs
//! - `div { "Content" }` - no attributes
//! - `div {}` - minimal
//! - `deferred { child }` - deferred element
//! - `Button::new("Hi") @[style: Primary] {}` - expression element
//! - `(complex + expr) @[style: Primary] {}` - parenthesized expression element

use proc_macro_error2::abort;
use proc_macro2::{Span, TokenStream, TokenTree};
use syn::ext::IdentExt;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::{Brace, Bracket, Paren};
use syn::{Expr, Ident, Result, Token, braced, bracketed};

use crate::ast::{
    Attribute, Child, ComponentElement, DeferredElement, Element, ExprElement, Markup,
    NativeElement,
};

const NATIVE_ELEMENTS: &[&str] = &["div", "svg", "anchored"];

/// Element head (the identifier or expression part before attributes/children)
enum ElementHead {
    /// Native element: div, svg, anchored
    Native(Ident),
    /// `deferred` element
    Deferred(Ident),
    /// Component (uppercase): Header, Footer
    Component(Ident),
    /// Expression element: s, `foo()`, `Button::new()`, (expr)
    Expression(Expr),
}

impl ElementHead {
    fn span(&self) -> proc_macro2::Span {
        match self {
            Self::Native(ident) | Self::Deferred(ident) | Self::Component(ident) => ident.span(),
            Self::Expression(expr) => expr.span(),
        }
    }
}

impl Parse for Markup {
    fn parse(input: ParseStream) -> Result<Self> {
        let element = parse_root_element(input)?;

        Ok(Self { element })
    }
}

/// Check if the input can continue as an expression (e.g. `div()`,
/// `Header::new()`)
fn can_continue_as_expr(input: ParseStream) -> bool {
    !input.is_empty() && input.parse::<Expr>().is_ok()
}

/// Parse the element head (identifier or expression before attributes/children)
fn parse_element_head(input: ParseStream) -> Result<ElementHead> {
    // arse parenthesized expression as Expression
    if input.peek(Paren) {
        let expr: Expr = input.parse()?;
        return Ok(ElementHead::Expression(expr));
    }

    if input.peek(Ident::peek_any) {
        let fork = input.fork();
        let ident = fork.call(Ident::parse_any)?;
        let name = ident.to_string();

        // Check if followed by element suffix (@ or {)
        let has_element_suffix = fork.peek(Token![@]) || fork.peek(Brace);

        // If not followed by element suffix, check if we need to parse as
        // expression
        if !has_element_suffix {
            // For known elements (native/deferred/component), check if it's
            // actually an expression
            let is_known = NATIVE_ELEMENTS.contains(&name.as_str())
                || name == "deferred"
                || name.starts_with(char::is_uppercase);

            if !is_known || can_continue_as_expr(&fork) {
                // For unknown lowercase idents or known elements with
                // expression continuation, parse as full
                // expression. This handles cases like `text.method()`, `foo()`,
                // `div()`, etc.
                let expr: Expr = input.parse()?;
                return Ok(ElementHead::Expression(expr));
            }
        }

        // known element
        let ident = input.call(Ident::parse_any)?;

        if NATIVE_ELEMENTS.contains(&name.as_str()) {
            return Ok(ElementHead::Native(ident));
        }
        if name == "deferred" {
            return Ok(ElementHead::Deferred(ident));
        }
        if name.starts_with(char::is_uppercase) {
            return Ok(ElementHead::Component(ident));
        }

        // Other lowercase identifiers
        let expr = Expr::Path(syn::ExprPath {
            attrs: vec![],
            qself: None,
            path: ident.into(),
        });

        return Ok(ElementHead::Expression(expr));
    }

    let expr: Expr = input.parse()?;

    Ok(ElementHead::Expression(expr))
}

/// Build an Element from the parsed head, attributes, and children
fn build_element(head: ElementHead, attributes: Vec<Attribute>, children: Vec<Child>) -> Element {
    match head {
        ElementHead::Native(name) => Element::Native(NativeElement {
            name,
            attributes,
            children,
        }),
        ElementHead::Deferred(name) => {
            if children.len() != 1 {
                abort!(name.span(), "deferred must have exactly one child");
            }
            Element::Deferred(DeferredElement {
                name,
                child: Box::new(children.into_iter().next().unwrap()),
            })
        }
        ElementHead::Component(name) => Element::Component(ComponentElement {
            name,
            attributes,
            children,
        }),
        ElementHead::Expression(expr) => Element::Expression(ExprElement {
            expr,
            attributes,
            children,
        }),
    }
}

/// Parse element children `{...}` with optional requirement
fn parse_element_children(
    input: ParseStream,
    require_braces: bool,
    head_span: Span,
) -> Result<Vec<Child>> {
    if !input.peek(Brace) {
        if require_braces {
            abort!(head_span, "element requires braces: `{}`");
        }
        return Ok(vec![]); // No children
    }

    let content;
    braced!(content in input);
    parse_children(&content)
}

/// Parse an element at the top level
fn parse_root_element(input: ParseStream) -> Result<Element> {
    let head = parse_element_head(input)?;
    let attributes = parse_attributes(input)?;
    let head_span = head.span();

    // Root element always requires braces
    if !input.peek(Brace) {
        abort!(
            head_span,
            "top-level element requires braces, e.g. `expr @[attrs] { children }`\n\
             note: braces declare this as a UI element in the component tree, not just an expression"
        );
    }

    let children = parse_element_children(input, true, head_span)?;

    Ok(build_element(head, attributes, children))
}

/// Parse attributes in `@[...]`
fn parse_attributes(input: ParseStream) -> Result<Vec<Attribute>> {
    if !input.peek(Token![@]) {
        return Ok(vec![]);
    }

    let at_token: Token![@] = input.parse()?;

    if !input.peek(Bracket) {
        abort!(
            at_token.span(),
            "expected `[` after `@` for attributes, e.g. `@[attr1, attr2]`"
        );
    }

    let content;
    bracketed!(content in input);

    parse_comma_separated(&content, parse_attribute)
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

/// Parse comma-separated children
fn parse_children(input: ParseStream) -> Result<Vec<Child>> {
    parse_comma_separated(input, parse_child)
}

fn parse_comma_separated<T>(
    input: ParseStream,
    parser: fn(ParseStream) -> Result<T>,
) -> Result<Vec<T>> {
    let items = Punctuated::<T, Token![,]>::parse_terminated_with(input, parser)?;

    Ok(items.into_iter().collect())
}

/// Parse a single child
fn parse_child(input: ParseStream) -> Result<Child> {
    if input.peek(Token![..]) {
        input.parse::<Token![..]>()?;
        return Ok(Child::Spread(input.parse()?));
    }

    if input.peek(Token![.]) {
        input.parse::<Token![.]>()?;
        return Ok(Child::MethodChain(parse_method_chain(input)?));
    }

    let head = parse_element_head(input)?;
    let attributes = parse_attributes(input)?;

    let require_braces = matches!(
        head,
        ElementHead::Native(_) | ElementHead::Deferred(_) | ElementHead::Component(_)
    );

    let children = parse_element_children(input, require_braces, head.span())?;

    let element = build_element(head, attributes, children);

    Ok(Child::Element(element))
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

        let tt = input.parse()?;

        // Track angle bracket depth for generics
        if let TokenTree::Punct(p) = &tt {
            match p.as_char() {
                '<' => angle_depth += 1,
                '>' => angle_depth = (angle_depth - 1).max(0),
                _ => {}
            }
        }

        tokens.extend([tt]);
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use quote::quote;
    use syn::parse2;

    use super::*;

    #[test]
    fn test_parse_simple_div() {
        let input = quote! { div {} };
        let markup: Markup = parse2(input).unwrap();
        assert!(matches!(markup.element, Element::Native(_)));
    }

    #[test]
    fn test_parse_div_with_attributes() {
        let input = quote! {
            div @[flex, w: px(200.0)] {}
        };
        let markup: Markup = parse2(input).unwrap();
        if let Element::Native(el) = markup.element {
            assert_eq!(el.attributes.len(), 2);
        } else {
            panic!("Expected Native element");
        }
    }

    #[test]
    fn test_parse_div_with_children() {
        let input = quote! {
            div {
                "Hello",
            }
        };
        let markup: Markup = parse2(input).unwrap();
        if let Element::Native(el) = markup.element {
            assert_eq!(el.children.len(), 1);
        } else {
            panic!("Expected Native element");
        }
    }

    #[test]
    fn test_parse_div_full() {
        let input = quote! {
            div @[flex, flex_col] {
                "Content",
                div @[bold] { "Nested" },
            }
        };
        let markup: Markup = parse2(input).unwrap();
        if let Element::Native(el) = markup.element {
            assert_eq!(el.attributes.len(), 2);
            assert_eq!(el.children.len(), 2);
        } else {
            panic!("Expected Native element");
        }
    }

    #[test]
    fn test_parse_expression_element() {
        let input = quote! {
            Container::new(title) {}
        };
        let markup: Markup = parse2(input).unwrap();
        assert!(matches!(markup.element, Element::Expression(_)));
    }

    #[test]
    fn test_parse_expression_element_with_attrs() {
        let input = quote! {
            Button::new("Click") @[style: Primary] {}
        };
        let markup: Markup = parse2(input).unwrap();
        if let Element::Expression(el) = markup.element {
            assert_eq!(el.attributes.len(), 1);
        } else {
            panic!("Expected Expression element");
        }
    }

    #[test]
    fn test_parse_spread_children() {
        let input = quote! {
            div {
                ..items,
            }
        };
        let markup: Markup = parse2(input).unwrap();
        if let Element::Native(el) = markup.element {
            assert_eq!(el.children.len(), 1);
            assert!(matches!(el.children[0], Child::Spread(_)));
        } else {
            panic!("Expected Native element");
        }
    }

    #[test]
    fn test_parse_method_chain() {
        let input = quote! {
            div {
                .when(cond, |d| d.flex()),
            }
        };
        let markup: Markup = parse2(input).unwrap();
        if let Element::Native(el) = markup.element {
            assert_eq!(el.children.len(), 1);
            assert!(matches!(el.children[0], Child::MethodChain(_)));
        } else {
            panic!("Expected Native element");
        }
    }

    #[test]
    fn test_parse_deferred() {
        let input = quote! {
            deferred {
                div { "Content" },
            }
        };
        let markup: Markup = parse2(input).unwrap();
        assert!(matches!(markup.element, Element::Deferred(_)));
    }

    #[test]
    fn test_parse_method_with_generics() {
        let input = quote! {
            div {
                .map::<Div, _>(|d| d),
            }
        };
        let markup: Markup = parse2(input).unwrap();
        if let Element::Native(el) = markup.element {
            assert!(matches!(el.children[0], Child::MethodChain(_)));
        } else {
            panic!("Expected Native element");
        }
    }

    #[test]
    fn test_parse_paren_child_without_attrs() {
        let input = quote! {
            div {
                (some_expr),
            }
        };
        let markup: Markup = parse2(input).unwrap();
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
    fn test_parse_paren_child_with_attrs() {
        let input = quote! {
            div {
                Button::new() @[flex] {},
            }
        };
        let markup: Markup = parse2(input).unwrap();
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
        let input = quote! { Header {} };
        let markup: Markup = parse2(input).unwrap();
        assert!(matches!(markup.element, Element::Component(_)));
    }

    #[test]
    fn test_parse_component_with_attrs() {
        let input = quote! {
            Header @[flex, style: Primary] {}
        };
        let markup: Markup = parse2(input).unwrap();
        if let Element::Component(el) = markup.element {
            assert_eq!(el.attributes.len(), 2);
        } else {
            panic!("Expected Component element");
        }
    }

    #[test]
    fn test_parse_component_with_children() {
        let input = quote! {
            Container {
                "Content",
                div { "Nested" },
            }
        };
        let markup: Markup = parse2(input).unwrap();
        if let Element::Component(el) = markup.element {
            assert_eq!(el.children.len(), 2);
        } else {
            panic!("Expected Component element");
        }
    }

    #[test]
    fn test_parse_component_child() {
        let input = quote! {
            div {
                Header @[flex] {},
            }
        };
        let markup: Markup = parse2(input).unwrap();
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
