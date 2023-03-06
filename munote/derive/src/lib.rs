// use proc_macro::TokenStream;
// use std::str::FromStr;
//
// use proc_macro2::{Span, TokenStream as TokenStream2};
// use quote::{format_ident, quote, ToTokens};
// use syn::{
//     parse_macro_input,
//     punctuated::Punctuated,
//     spanned::Spanned,
//     Attribute,
//     Data,
//     DeriveInput,
//     Error,
//     Field,
//     Lit,
//     Meta,
//     NestedMeta,
//     Token,
// };
//
// use crate::{attribute_name::AttributeName, field_attribute_name::FieldAttributeName, when::When};
//
// #[proc_macro_derive(Symbol)]
// pub fn derive(input: TokenStream) -> TokenStream {
//     let input = parse_macro_input!(input as DeriveInput);
//
//     let gen = match derive_impl(input) {
//         Ok(gen) => gen,
//         Err(err) => err.to_compile_error(),
//     };
//
//     gen.into()
// }
//
// fn derive_impl(input: DeriveInput) -> Result<TokenStream2, Error> {
//     let ident = input.ident;
//     let name = ident.to_string();
//
//     let expanded = quote! {
//         impl crate::Symbol for Note {
//             fn as_any(&self) -> &dyn Any {
//                 self
//             }
//
//             fn equals(&self, other: &dyn Symbol) -> bool {
//                 other
//                     .as_any()
//                     .downcast_ref::<Self>()
//                     .map_or(false, |a| self == a)
//             }
//
//             fn octave(&self) -> i8 {
//                 self.octave
//             }
//
//             fn duration(&self) -> Duration {
//                 self.duration
//             }
//         }
//     };
//
//     // eprintln!("Expanded: {}", expanded);
//
//     Ok(expanded)
// }
