use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(TolTokenKind, attributes(keyword, stmt_starter, semicolon_inferrable))]
pub fn derive_tokenkind(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let data_enum = match input.data {
        syn::Data::Enum(e) => e,
        _ => panic!("Token only works on enums"),
    };

    let enum_name = &input.ident;
    let mut keyword_arms = Vec::new();
    let mut keyword_variants = Vec::new();
    let mut stmt_starter_variants = Vec::new();
    let mut inferrable_variants = Vec::new();

    for variant in &data_enum.variants {
        let variant_name = &variant.ident;

        let has_keyword = variant
            .attrs
            .iter()
            .any(|attr| attr.path().is_ident("keyword"));
        let has_stmt_starter = variant
            .attrs
            .iter()
            .any(|attr| attr.path().is_ident("stmt_starter"));
        let has_semicolon_inferrable = variant
            .attrs
            .iter()
            .any(|attr| attr.path().is_ident("semicolon_inferrable"));

        if has_keyword {
            let kw_str = variant_name.to_string().to_lowercase();
            keyword_arms.push(quote! {
                #kw_str => Some(Self::#variant_name)
            });
            keyword_variants.push(quote! {
                Self::#variant_name
            });
        }

        if has_stmt_starter {
            stmt_starter_variants.push(quote! {
                Self::#variant_name
            });
        }

        if has_semicolon_inferrable {
            inferrable_variants.push(quote! {
                Self::#variant_name
            });
        }
    }

    let is_keyword_body = if keyword_variants.is_empty() {
        quote! {false}
    } else {
        quote! {
            matches!(self, #(#keyword_variants)|*)
        }
    };
    let starts_a_statement_body = if stmt_starter_variants.is_empty() {
        quote! {false}
    } else {
        quote! {matches!(self, #(#stmt_starter_variants)|*)}
    };
    let is_semicolon_inferrable_body = if inferrable_variants.is_empty() {
        quote! {false}
    } else {
        quote! {matches!(self, #(#inferrable_variants)|*)}
    };
    let expanded = quote! {
        impl #enum_name {
            /// Returns Some(Self) if it is a keyword or None from a given &str
            pub fn from_keyword(s: &str) -> Option<Self> {
                match s {
                    #(#keyword_arms,)*
                    _ => None,
                }
            }

            pub fn is_keyword(&self) -> bool {
                #is_keyword_body
            }

            pub fn starts_a_statement(&self) -> bool {
                #starts_a_statement_body
            }

            pub fn is_semicolon_inferrable(&self) -> bool {
                #is_semicolon_inferrable_body
            }
        }
    };

    TokenStream::from(expanded)
}
