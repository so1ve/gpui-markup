//! Code generation for gpui-markup DSL.

use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote};
use syn::Ident;
use syn::spanned::Spanned;

use crate::ast::{Attribute, Child, Element, Markup};

impl ToTokens for Markup {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(generate_element(&self.element, &self.errors));
    }
}

fn element_span(element: &Element) -> Span {
    match element {
        Element::Native(el) => el.name.span(),
        Element::Component(el) => el.name.span(),
        Element::Deferred(el) => el.name.span(),
        Element::Expression(el) => el.expr.span(),
    }
}

fn generate_element(element: &Element, errors: &[syn::Error]) -> TokenStream {
    let (base, attributes, children) = match element {
        Element::Native(el) => {
            let name = &el.name;

            (
                quote! { #name() },
                el.attributes.as_slice(),
                el.children.as_deref(),
            )
        }
        Element::Component(el) => {
            let name = &el.name;

            (
                quote! { #name::new() },
                el.attributes.as_slice(),
                el.children.as_deref(),
            )
        }
        Element::Expression(el) => {
            let expr = &el.expr;

            (
                quote! { #expr },
                el.attributes.as_slice(),
                el.children.as_deref(),
            )
        }
        Element::Deferred(el) => {
            let name = &el.name;
            let child = generate_element(&el.child, &[]);

            (
                quote! { #name(gpui::IntoElement::into_any_element(#child)) },
                &[][..],
                None,
            )
        }
    };
    let base = append_attributes(base, attributes);
    // A bare child expression stays bare. A body, even an empty one, gets a
    // block so adding its first child does not change the preceding expansion.
    if children.is_none() && errors.is_empty() {
        return base;
    }
    // Keep the expansion in source order: adding a child must not insert a
    // UFCS prefix ahead of every earlier token. rust-analyzer compares the
    // normal expansion with one containing a completion identifier.
    let value = Ident::new(
        "__gpui_markup_element",
        Span::mixed_site().located_at(element_span(element)),
    );
    let children = children.into_iter().flatten().map(|child| match child {
        Child::Element(element) => {
            let child_value = Ident::new(
                "__child",
                Span::mixed_site().located_at(element_span(element)),
            );
            let element = generate_element(element, &[]);

            quote! {
                let #child_value = #element;
                let #value = gpui::ParentElement::child(#value, #child_value);
            }
        }
        Child::Spread(expr) => {
            let child_value = Ident::new("__child", Span::mixed_site().located_at(expr.span()));

            quote! {
                let #child_value = #expr;
                let #value = gpui::ParentElement::children(#value, #child_value);
            }
        }
        Child::MethodChain(chain) => quote! {
            let #value = #value.#chain;
        },
    });
    let errors = errors.iter().map(syn::Error::to_compile_error);

    // Emit diagnostics after the tree so an unfinished node does not shift
    // the source tokens or erase the expression used for completion.
    quote! {{
        let #value = #base;
        #(#children)*
        #(#errors)*
        #value
    }}
}

fn append_attributes(output: TokenStream, attributes: &[Attribute]) -> TokenStream {
    attributes.iter().fold(output, |acc, attr| match attr {
        Attribute::Flag(name) => quote! { #acc.#name() },
        Attribute::KeyValue { key, value } => {
            if let syn::Expr::Tuple(tuple) = value {
                let elems = &tuple.elems;

                quote! { #acc.#key(#elems) }
            } else {
                quote! { #acc.#key(#value) }
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use insta::assert_snapshot;

    use super::*;
    use crate::ast::Markup;

    fn generate(input: proc_macro2::TokenStream) -> String {
        let markup: Markup = syn::parse2(input).unwrap();
        let output = quote! { fn __wrapper() { #markup } };
        let syntax_tree = syn::parse_file(&output.to_string()).unwrap();

        prettyplease::unparse(&syntax_tree)
    }

    #[test]
    fn test_simple_div() {
        assert_snapshot!(generate(quote::quote! { div {} }));
    }

    #[test]
    fn test_div_with_flag_attribute() {
        assert_snapshot!(generate(quote::quote! { div @[flex] {} }));
    }

    #[test]
    fn test_div_with_multiple_flags() {
        assert_snapshot!(generate(quote::quote! { div @[flex, flex_col] {} }));
    }

    #[test]
    fn test_div_with_key_value_attribute() {
        assert_snapshot!(generate(quote::quote! { div @[w: px(200.0)] {} }));
    }

    #[test]
    fn test_div_with_mixed_attributes() {
        assert_snapshot!(generate(
            quote::quote! { div @[flex, w: px(200.0), bg: theme.secondary] {} }
        ));
    }

    #[test]
    fn test_div_with_single_child() {
        assert_snapshot!(generate(quote::quote! { div { "Hello" } }));
    }

    #[test]
    fn test_div_with_multiple_children() {
        assert_snapshot!(generate(quote::quote! {
            div {
                "First",
                "Second",
            }
        }));
    }

    #[test]
    fn test_div_with_element_children() {
        assert_snapshot!(generate(quote::quote! {
            div {
                div { "First" },
                div { "Second" },
            }
        }));
    }

    #[test]
    fn test_expression_element() {
        assert_snapshot!(generate(quote::quote! { Container::new(title) {} }));
    }

    #[test]
    fn test_expression_element_with_attributes() {
        assert_snapshot!(generate(quote::quote! { Container::new(title) @[flex] {} }));
    }

    #[test]
    fn test_expression_element_with_children() {
        assert_snapshot!(generate(
            quote::quote! { Container::new(title) { "Content" } }
        ));
    }

    #[test]
    fn test_nested_elements() {
        assert_snapshot!(generate(quote::quote! {
            div @[flex] {
                div { "Inner" },
            }
        }));
    }

    #[test]
    fn test_multi_value_attribute() {
        assert_snapshot!(generate(
            quote::quote! { div @[when: (is_visible, |d| d.flex())] {} }
        ));
    }

    #[test]
    fn test_complex_nested() {
        assert_snapshot!(generate(quote::quote! {
            div @[flex, flex_col] {
                div @[text_size: px(16.0)] {
                    "Hello World",
                },
                div @[bg: theme.secondary] {
                    (Header::new()),
                },
            }
        }));
    }

    #[test]
    fn test_svg() {
        assert_snapshot!(generate(
            quote::quote! { svg @[path: icon_path, size: px(24.0)] {} }
        ));
    }

    #[test]
    fn test_anchored() {
        assert_snapshot!(generate(quote::quote! {
            anchored @[position: Point::default()] {
                div { "Tooltip" },
            }
        }));
    }

    #[test]
    fn test_deferred() {
        assert_snapshot!(generate(quote::quote! {
            deferred {
                div { "Deferred content" },
            }
        }));
    }

    #[test]
    fn test_spread_children() {
        assert_snapshot!(generate(quote::quote! {
            div {
                ..items,
            }
        }));
    }

    #[test]
    fn test_spread_with_siblings() {
        assert_snapshot!(generate(quote::quote! {
            div {
                "Header",
                ..items,
                "Footer",
            }
        }));
    }

    #[test]
    fn test_method_call() {
        assert_snapshot!(generate(quote::quote! {
            div {
                "static",
                .when(cond, |d| d.child("dynamic")),
            }
        }));
    }

    #[test]
    fn test_method_call_no_args() {
        assert_snapshot!(generate(quote::quote! {
            div {
                .flex(),
            }
        }));
    }

    #[test]
    fn test_method_chain() {
        assert_snapshot!(generate(quote::quote! {
            div {
                .flex().flex_col().gap_2(),
            }
        }));
    }

    #[test]
    fn test_method_with_generics() {
        assert_snapshot!(generate(quote::quote! {
            div {
                .map::<Div, _>(|d| d),
            }
        }));
    }

    #[test]
    fn test_component() {
        assert_snapshot!(generate(quote::quote! { Header {} }));
    }

    #[test]
    fn test_component_with_attrs() {
        assert_snapshot!(generate(
            quote::quote! { Header @[flex, style: Primary] {} }
        ));
    }

    #[test]
    fn test_component_with_children() {
        assert_snapshot!(generate(quote::quote! {
            Container {
                "Content",
            }
        }));
    }

    #[test]
    fn test_component_nested() {
        assert_snapshot!(generate(quote::quote! {
            div {
                Header @[flex] {},
                Footer {},
            }
        }));
    }
}
