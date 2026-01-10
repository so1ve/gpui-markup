//! Parser for gpui-markup DSL.

use proc_macro_error2::abort;
use syn::ext::IdentExt;
use syn::parse::{Parse, ParseStream};
use syn::token::Brace;
use syn::{Expr, Ident, Result, Token, braced};

use crate::ast::{Attribute, Child, ComponentElement, Element, ExprElement, Markup, NativeElement};

/// Native GPUI element names
const NATIVE_ELEMENTS: &[&str] = &["div", "img", "svg", "canvas", "anchored"];

impl Parse for Markup {
    fn parse(input: ParseStream) -> Result<Self> {
        let element = input.parse::<Element>()?;
        Ok(Self { element })
    }
}

impl Parse for Element {
    fn parse(input: ParseStream) -> Result<Self> {
        // Expect `<` to start a tag
        input.parse::<Token![<]>()?;

        if input.peek(Brace) {
            // <{expr}> - expression tag
            parse_expression_element(input)
        } else {
            // <ident> - native element or component
            let ident = input.call(Ident::parse_any)?;
            let name = ident.to_string();

            if NATIVE_ELEMENTS.contains(&name.as_str()) {
                parse_native_element(input, ident)
            } else if is_component_name(&name) {
                parse_component_element(input, ident)
            } else {
                abort!(
                    ident.span(),
                    "Unknown element '{}'. Use native elements (div, img, svg, canvas, anchored) or PascalCase for components.",
                    name
                );
            }
        }
    }
}

/// Check if the name is a component (starts with uppercase)
fn is_component_name(name: &str) -> bool {
    name.chars().next().is_some_and(|c| c.is_ascii_uppercase())
}

fn parse_native_element(input: ParseStream, open_name: Ident) -> Result<Element> {
    let tag_name = open_name.to_string();
    let attributes = parse_attributes(input)?;

    // Check for self-closing or opening tag
    if input.peek(Token![/]) {
        // Self-closing: <tag .../>
        input.parse::<Token![/]>()?;
        input.parse::<Token![>]>()?;
        return Ok(Element::Native(NativeElement {
            open_name,
            close_name: None,
            attributes,
            children: vec![],
        }));
    }

    // Opening tag: <tag ...>
    input.parse::<Token![>]>()?;

    // Parse children
    let children = parse_children(input)?;

    // Closing tag: </tag>
    input.parse::<Token![<]>()?;
    input.parse::<Token![/]>()?;
    let closing_ident = input.call(Ident::parse_any)?;
    if closing_ident != tag_name {
        abort!(
            closing_ident.span(),
            "Mismatched closing tag. Expected </{}>, found </{}>",
            tag_name,
            closing_ident
        );
    }
    input.parse::<Token![>]>()?;

    Ok(Element::Native(NativeElement {
        open_name,
        close_name: Some(closing_ident),
        attributes,
        children,
    }))
}

/// Parse a component element: `<Foo/>` or `<Foo>...</Foo>`
fn parse_component_element(input: ParseStream, open_name: Ident) -> Result<Element> {
    // Check for self-closing or opening tag
    if input.peek(Token![/]) {
        // Self-closing: <Component/>
        input.parse::<Token![/]>()?;
        input.parse::<Token![>]>()?;
        return Ok(Element::Component(ComponentElement {
            open_name,
            close_name: None,
            children: vec![],
        }));
    }

    // Opening tag: <Component>
    input.parse::<Token![>]>()?;

    // Parse children
    let children = parse_children(input)?;

    // Closing tag: </Component>
    input.parse::<Token![<]>()?;
    input.parse::<Token![/]>()?;
    let closing_ident = input.call(Ident::parse_any)?;
    if closing_ident != open_name {
        abort!(
            closing_ident.span(),
            "Mismatched closing tag. Expected </{}>, found </{}>",
            open_name,
            closing_ident
        );
    }
    input.parse::<Token![>]>()?;

    Ok(Element::Component(ComponentElement {
        open_name,
        close_name: Some(closing_ident),
        children,
    }))
}

/// Parse an expression element: `<{expr}/>` or `<{expr} ...>...</{}>`
fn parse_expression_element(input: ParseStream) -> Result<Element> {
    // Parse {expr}
    let content;
    braced!(content in input);
    let expr: Expr = content.parse()?;

    let attributes = parse_attributes(input)?;

    // Check for self-closing or opening tag
    if input.peek(Token![/]) {
        // Self-closing: <{expr}/>
        input.parse::<Token![/]>()?;
        input.parse::<Token![>]>()?;
        return Ok(Element::Expression(ExprElement {
            expr,
            attributes,
            children: vec![],
        }));
    }

    // Opening tag: <{expr}>
    input.parse::<Token![>]>()?;

    let children = parse_children(input)?;

    // Closing tag: </{}>
    input.parse::<Token![<]>()?;
    input.parse::<Token![/]>()?;
    let closing_content;
    braced!(closing_content in input);
    // The closing brace should be empty
    if !closing_content.is_empty() {
        abort!(
            closing_content.span(),
            "Closing tag for expression elements should be empty: </{{}}>"
        );
    }
    input.parse::<Token![>]>()?;

    Ok(Element::Expression(ExprElement {
        expr,
        attributes,
        children,
    }))
}

/// Parse attributes until we hit `>` or `/>`
fn parse_attributes(input: ParseStream) -> Result<Vec<Attribute>> {
    let mut attributes = Vec::new();

    while !input.peek(Token![>]) && !input.peek(Token![/]) {
        let attr = parse_attribute(input)?;
        attributes.push(attr);
    }

    Ok(attributes)
}

/// Parse a single attribute: `flex`, `w={expr}`, or `when={expr1, expr2}`
fn parse_attribute(input: ParseStream) -> Result<Attribute> {
    let key = input.call(Ident::parse_any)?;

    if input.peek(Token![=]) {
        input.parse::<Token![=]>()?;
        let content;
        braced!(content in input);

        // Parse comma-separated expressions
        let mut values: Vec<Expr> = Vec::new();
        values.push(content.parse()?);

        while content.peek(Token![,]) {
            content.parse::<Token![,]>()?;
            if !content.is_empty() {
                values.push(content.parse()?);
            }
        }

        Ok(Attribute::KeyValue { key, values })
    } else {
        Ok(Attribute::Flag(key))
    }
}

/// Parse children until we hit a closing tag `</`
fn parse_children(input: ParseStream) -> Result<Vec<Child>> {
    let mut children = Vec::new();

    while !is_closing_tag(input) {
        let child = parse_child(input)?;
        children.push(child);
    }

    Ok(children)
}

/// Check if we're at a closing tag `</`
fn is_closing_tag(input: ParseStream) -> bool {
    input.peek(Token![<]) && input.peek2(Token![/])
}

/// Parse a single child: `{expr}` or nested element
fn parse_child(input: ParseStream) -> Result<Child> {
    if input.peek(Token![<]) {
        // Nested element
        let element = input.parse::<Element>()?;
        Ok(Child::Element(element))
    } else if input.peek(Brace) {
        // Expression child
        let content;
        braced!(content in input);
        let expr: Expr = content.parse()?;
        Ok(Child::Expression(expr))
    } else {
        abort!(
            input.span(),
            "Expected a child element `<...>` or expression `{...}`"
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_div() {
        let input: proc_macro2::TokenStream = quote::quote! {
            <div/>
        };
        let markup: Markup = syn::parse2(input).unwrap();
        assert!(matches!(markup.element, Element::Native(_)));
    }

    #[test]
    fn test_parse_div_with_attributes() {
        let input: proc_macro2::TokenStream = quote::quote! {
            <div flex w={px(200.0)}/>
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
            <div>
                {"Hello"}
            </div>
        };
        let markup: Markup = syn::parse2(input).unwrap();
        if let Element::Native(el) = markup.element {
            assert_eq!(el.children.len(), 1);
        } else {
            panic!("Expected Native element");
        }
    }

    #[test]
    fn test_parse_img() {
        let input: proc_macro2::TokenStream = quote::quote! {
            <img src={image_source}/>
        };
        let markup: Markup = syn::parse2(input).unwrap();
        if let Element::Native(el) = markup.element {
            assert_eq!(el.open_name.to_string(), "img");
        } else {
            panic!("Expected Native element");
        }
    }

    #[test]
    fn test_parse_svg() {
        let input: proc_macro2::TokenStream = quote::quote! {
            <svg path={icon_path}/>
        };
        let markup: Markup = syn::parse2(input).unwrap();
        if let Element::Native(el) = markup.element {
            assert_eq!(el.open_name.to_string(), "svg");
        } else {
            panic!("Expected Native element");
        }
    }

    #[test]
    fn test_parse_component() {
        let input: proc_macro2::TokenStream = quote::quote! {
            <Header/>
        };
        let markup: Markup = syn::parse2(input).unwrap();
        if let Element::Component(comp) = markup.element {
            assert_eq!(comp.open_name.to_string(), "Header");
        } else {
            panic!("Expected Component element");
        }
    }

    #[test]
    fn test_parse_component_with_children() {
        let input: proc_macro2::TokenStream = quote::quote! {
            <Container>
                {"Content"}
            </Container>
        };
        let markup: Markup = syn::parse2(input).unwrap();
        if let Element::Component(comp) = markup.element {
            assert_eq!(comp.open_name.to_string(), "Container");
            assert_eq!(comp.children.len(), 1);
        } else {
            panic!("Expected Component element");
        }
    }

    #[test]
    fn test_parse_expression_element() {
        let input: proc_macro2::TokenStream = quote::quote! {
            <{Container::new(title)}/>
        };
        let markup: Markup = syn::parse2(input).unwrap();
        assert!(matches!(markup.element, Element::Expression(_)));
    }
}
