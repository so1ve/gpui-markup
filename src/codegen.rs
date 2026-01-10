//! Code generation for gpui-markup DSL.

use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

use crate::ast::{
    Attribute, Child, ComponentElement, DeferredElement, Element, ExprElement, Markup,
    NativeElement,
};

impl ToTokens for Markup {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.element.to_tokens(tokens);
    }
}

impl ToTokens for Element {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Native(el) => el.to_tokens(tokens),
            Self::Component(el) => el.to_tokens(tokens),
            Self::Deferred(el) => el.to_tokens(tokens),
            Self::Expression(expr) => expr.to_tokens(tokens),
        }
    }
}

impl ToTokens for NativeElement {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.name;
        let mut output = quote! { #name() };

        output = append_attributes(output, &self.attributes);
        output = append_children(output, &self.children);

        tokens.extend(output);
    }
}

impl ToTokens for ComponentElement {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.name;
        let mut output = quote! { #name::new() };

        output = append_attributes(output, &self.attributes);
        output = append_children(output, &self.children);

        tokens.extend(output);
    }
}

impl ToTokens for DeferredElement {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.name;
        let child_tokens = match self.child.as_ref() {
            Child::Element(element) => quote! { #element },
            Child::Expression(expr) => quote! { #expr },
            Child::Spread(_) | Child::MethodChain(_) => {
                unreachable!("deferred only accepts Element or Expression")
            }
        };
        let output = quote! { #name((#child_tokens).into_any_element()) };
        tokens.extend(output);
    }
}

impl ToTokens for ExprElement {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let expr = &self.expr;
        let mut output = append_attributes(quote! { #expr }, &self.attributes);
        output = append_children(output, &self.children);
        tokens.extend(output);
    }
}

fn append_attributes(mut output: TokenStream, attributes: &[Attribute]) -> TokenStream {
    for attr in attributes {
        output = match attr {
            Attribute::Flag(name) => quote! { #output.#name() },
            Attribute::KeyValue { key, value } => {
                // Check if value is a tuple - if so, expand its elements as separate arguments
                if let syn::Expr::Tuple(tuple) = value {
                    let elems = &tuple.elems;
                    quote! { #output.#key(#elems) }
                } else {
                    quote! { #output.#key(#value) }
                }
            }
        };
    }
    output
}

fn append_children(mut output: TokenStream, children: &[Child]) -> TokenStream {
    for child in children {
        output = match child {
            Child::Element(element) => quote! { #output.child(#element) },
            Child::Expression(expr) => quote! { #output.child(#expr) },
            Child::Spread(expr) => quote! { #output.children(#expr) },
            Child::MethodChain(tokens) => quote! { #output.#tokens },
        };
    }
    output
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
        assert_snapshot!(generate(quote::quote! { div { [flex] } }));
    }

    #[test]
    fn test_div_with_multiple_flags() {
        assert_snapshot!(generate(quote::quote! { div { [flex, flex_col] } }));
    }

    #[test]
    fn test_div_with_key_value_attribute() {
        assert_snapshot!(generate(quote::quote! { div { [w: px(200.0)] } }));
    }

    #[test]
    fn test_div_with_mixed_attributes() {
        assert_snapshot!(generate(
            quote::quote! { div { [flex, w: px(200.0), bg: theme.secondary] } }
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
        assert_snapshot!(generate(quote::quote! { Container::new(title) { [flex] } }));
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
            div { [flex]
                div { "Inner" },
            }
        }));
    }

    #[test]
    fn test_multi_value_attribute() {
        assert_snapshot!(generate(
            quote::quote! { div { [when: (is_visible, |d| d.flex())] } }
        ));
    }

    #[test]
    fn test_complex_nested() {
        assert_snapshot!(generate(quote::quote! {
            div { [flex, flex_col]
                div { [text_size: px(16.0)]
                    "Hello World",
                },
                div { [bg: theme.secondary]
                    (Header::new()),
                },
            }
        }));
    }

    #[test]
    fn test_svg() {
        assert_snapshot!(generate(
            quote::quote! { svg { [path: icon_path, size: px(24.0)] } }
        ));
    }

    #[test]
    fn test_anchored() {
        assert_snapshot!(generate(quote::quote! {
            anchored { [position: Point::default()]
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
            quote::quote! { Header { [flex, style: Primary] } }
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
                Header { [flex] },
                Footer {},
            }
        }));
    }
}
