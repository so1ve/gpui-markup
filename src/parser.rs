//! Parser for gpui-markup DSL.

use proc_macro_error2::abort;
use syn::ext::IdentExt;
use syn::parse::{Parse, ParseStream};
use syn::token::Brace;
use syn::{Expr, Ident, Result, Token, braced};

use crate::ast::{Attribute, Child, ComponentElement, DivElement, Element, ExprElement, Markup};

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
            // <ident> - div or component
            let ident = input.call(Ident::parse_any)?;
            let name = ident.to_string();

            if name == "div" {
                parse_div_element(input)
            } else if is_component_name(&name) {
                parse_component_element(input, ident)
            } else {
                abort!(
                    ident.span(),
                    "Unknown element '{}'. Use 'div' for native elements or PascalCase for components.",
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

fn parse_div_element(input: ParseStream) -> Result<Element> {
    let attributes = parse_attributes(input)?;

    // Check for self-closing or opening tag
    if input.peek(Token![/]) {
        // Self-closing: <div .../>
        input.parse::<Token![/]>()?;
        input.parse::<Token![>]>()?;
        return Ok(Element::Div(DivElement {
            attributes,
            children: vec![],
        }));
    }

    // Opening tag: <div ...>
    input.parse::<Token![>]>()?;

    // Parse children
    let children = parse_children(input)?;

    // Closing tag: </div>
    input.parse::<Token![<]>()?;
    input.parse::<Token![/]>()?;
    let closing_ident = input.call(Ident::parse_any)?;
    if closing_ident != "div" {
        abort!(
            closing_ident.span(),
            "Mismatched closing tag. Expected </div>, found </{}>",
            closing_ident
        );
    }
    input.parse::<Token![>]>()?;

    Ok(Element::Div(DivElement {
        attributes,
        children,
    }))
}

/// Parse a component element: `<Foo/>` (only self-closing for now)
fn parse_component_element(input: ParseStream, name: Ident) -> Result<Element> {
    // Components must be self-closing
    if !input.peek(Token![/]) {
        abort!(
            name.span(),
            "Components must be self-closing: <{}/>. For components with children, use expression syntax: <{{{}::new(...)}}>",
            name,
            name
        );
    }

    input.parse::<Token![/]>()?;
    input.parse::<Token![>]>()?;

    Ok(Element::Component(ComponentElement { name }))
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
        // Key-value or multi-value attribute
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

        if values.len() == 1 {
            Ok(Attribute::KeyValue {
                key,
                value: values.pop().unwrap(),
            })
        } else {
            Ok(Attribute::KeyMultiValue { key, values })
        }
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
        assert!(matches!(markup.element, Element::Div(_)));
    }

    #[test]
    fn test_parse_div_with_attributes() {
        let input: proc_macro2::TokenStream = quote::quote! {
            <div flex w={px(200.0)}/>
        };
        let markup: Markup = syn::parse2(input).unwrap();
        if let Element::Div(div) = markup.element {
            assert_eq!(div.attributes.len(), 2);
        } else {
            panic!("Expected Div element");
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
        if let Element::Div(div) = markup.element {
            assert_eq!(div.children.len(), 1);
        } else {
            panic!("Expected Div element");
        }
    }

    #[test]
    fn test_parse_component() {
        let input: proc_macro2::TokenStream = quote::quote! {
            <Header/>
        };
        let markup: Markup = syn::parse2(input).unwrap();
        if let Element::Component(comp) = markup.element {
            assert_eq!(comp.name.to_string(), "Header");
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
