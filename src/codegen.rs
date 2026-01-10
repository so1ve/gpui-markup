//! Code generation for gpui-markup DSL.

use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::Ident;

use crate::ast::{Attribute, Child, ComponentElement, Element, ExprElement, Markup, NativeElement};

fn native_element_type(name: &str) -> &'static str {
    match name {
        "div" => "Div",
        "img" => "Img",
        "svg" => "Svg",
        "canvas" => "Canvas",
        "anchored" => "Anchored",
        _ => unreachable!("Unknown native element: {name}"),
    }
}

/// Generate base output with span preservation for open/close tags
fn generate_base_with_spans(
    open_name: &Ident,
    close_name: Option<&Ident>,
    constructor: impl FnOnce(&Ident) -> TokenStream,
) -> TokenStream {
    if let Some(close_name) = close_name {
        let ctor = constructor(close_name);
        quote! {
            {
                #[allow(path_statements)]
                #open_name;
                #ctor
            }
        }
    } else {
        constructor(open_name)
    }
}

fn wrap_with_parent_check(output: &TokenStream, type_path: &TokenStream) -> TokenStream {
    quote! {
        {
            fn __assert_parent_element<T: gpui::ParentElement>() {}
            __assert_parent_element::<#type_path>();
            #output
        }
    }
}

impl ToTokens for Markup {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.element.to_tokens(tokens);
    }
}

impl ToTokens for Element {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Native(el) => el.to_tokens(tokens),
            Self::Component(comp) => comp.to_tokens(tokens),
            Self::Expression(expr) => expr.to_tokens(tokens),
        }
    }
}

impl ToTokens for NativeElement {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut output = generate_base_with_spans(
            &self.open_name,
            self.close_name.as_ref(),
            |name| quote! { #name() },
        );

        output = append_attributes(output, &self.attributes);

        if !self.children.is_empty() {
            let type_name =
                quote::format_ident!("{}", native_element_type(&self.open_name.to_string()));
            output = wrap_with_parent_check(&output, &quote! { gpui::#type_name });
            output = append_children(output, &self.children);
        }

        tokens.extend(output);
    }
}

impl ToTokens for ComponentElement {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut output = generate_base_with_spans(
            &self.open_name,
            self.close_name.as_ref(),
            |name| quote! { #name::new() },
        );

        if !self.children.is_empty() {
            let open_name = &self.open_name;
            output = wrap_with_parent_check(&output, &quote! { #open_name });
            output = append_children(output, &self.children);
        }

        tokens.extend(output);
    }
}

impl ToTokens for ExprElement {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let expr = &self.expr;

        let mut output = quote! { #expr };

        output = append_attributes(output, &self.attributes);

        output = append_children(output, &self.children);

        tokens.extend(output);
    }
}

impl ToTokens for Child {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Element(element) => element.to_tokens(tokens),
            Self::Expression(expr) => expr.to_tokens(tokens),
        }
    }
}

fn append_attributes(mut output: TokenStream, attributes: &[Attribute]) -> TokenStream {
    for attr in attributes {
        output = match attr {
            Attribute::Flag(name) => quote! { #output.#name() },
            Attribute::KeyValue { key, values } => quote! { #output.#key(#(#values),*) },
        };
    }
    output
}

fn append_children(output: TokenStream, children: &[Child]) -> TokenStream {
    match children.len() {
        0 => output,
        1 => {
            let child = &children[0];
            quote! { #output.child(#child) }
        }
        _ => {
            let children_tokens: Vec<_> = children
                .iter()
                .map(|c| quote! { (#c).into_any_element() })
                .collect();
            quote! { #output.children([#(#children_tokens),*]) }
        }
    }
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
        assert_snapshot!(generate(quote::quote! { <div/> }));
    }

    #[test]
    fn test_div_with_flag_attribute() {
        assert_snapshot!(generate(quote::quote! { <div flex/> }));
    }

    #[test]
    fn test_div_with_multiple_flags() {
        assert_snapshot!(generate(quote::quote! { <div flex flex_col/> }));
    }

    #[test]
    fn test_div_with_key_value_attribute() {
        assert_snapshot!(generate(quote::quote! { <div w={px(200.0)}/> }));
    }

    #[test]
    fn test_div_with_mixed_attributes() {
        assert_snapshot!(generate(
            quote::quote! { <div flex w={px(200.0)} bg={theme.secondary}/> }
        ));
    }

    #[test]
    fn test_div_with_single_child() {
        assert_snapshot!(generate(quote::quote! { <div>{"Hello"}</div> }));
    }

    #[test]
    fn test_div_with_multiple_children() {
        assert_snapshot!(generate(quote::quote! {
            <div>
                {"First"}
                {"Second"}
            </div>
        }));
    }

    #[test]
    fn test_div_with_element_children() {
        assert_snapshot!(generate(quote::quote! {
            <div>
                <div>{"First"}</div>
                <div>{"Second"}</div>
            </div>
        }));
    }

    #[test]
    fn test_component() {
        assert_snapshot!(generate(quote::quote! { <Header/> }));
    }

    #[test]
    fn test_component_with_children() {
        assert_snapshot!(generate(quote::quote! {
            <Container>
                <div>{"Content"}</div>
            </Container>
        }));
    }

    #[test]
    fn test_expression_element() {
        assert_snapshot!(generate(quote::quote! { <{Container::new(title)}/> }));
    }

    #[test]
    fn test_expression_element_with_attributes() {
        assert_snapshot!(generate(quote::quote! { <{Container::new(title)} flex/> }));
    }

    #[test]
    fn test_expression_element_with_children() {
        assert_snapshot!(generate(
            quote::quote! { <{Container::new(title)}>{"Content"}</{}>  }
        ));
    }

    #[test]
    fn test_nested_elements() {
        assert_snapshot!(generate(quote::quote! {
            <div flex>
                <div>{"Inner"}</div>
            </div>
        }));
    }

    #[test]
    fn test_multi_value_attribute() {
        assert_snapshot!(generate(
            quote::quote! { <div when={is_visible, |d| d.flex()}/> }
        ));
    }

    #[test]
    fn test_complex_nested() {
        assert_snapshot!(generate(quote::quote! {
            <div flex flex_col>
                <div text_size={px(16.0)}>
                    {"Hello World"}
                </div>
                <div bg={theme.secondary}>
                    <Header/>
                </div>
            </div>
        }));
    }

    #[test]
    fn test_img() {
        assert_snapshot!(generate(quote::quote! { <img src={image_source}/> }));
    }

    #[test]
    fn test_svg() {
        assert_snapshot!(generate(
            quote::quote! { <svg path={icon_path} size={px(24.0)}/> }
        ));
    }

    #[test]
    fn test_canvas() {
        assert_snapshot!(generate(
            quote::quote! { <canvas w={px(100.0)} h={px(100.0)}/> }
        ));
    }

    #[test]
    fn test_anchored() {
        assert_snapshot!(generate(quote::quote! {
            <anchored position={Point::default()}>
                <div>{"Tooltip"}</div>
            </anchored>
        }));
    }
}
