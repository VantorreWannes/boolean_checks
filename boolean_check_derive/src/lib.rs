extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input, parse_quote};

#[proc_macro_derive(CheckOps)]
pub fn check_ops_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = &input.ident;

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    // --- Generics for `BitAnd`/`BitOr` on a value ---
    let mut generics_with_rhs = input.generics.clone();
    generics_with_rhs
        .params
        .push(parse_quote!(Rhs: crate::Check));
    let (impl_generics_rhs, _, where_clause_rhs) = generics_with_rhs.split_for_impl();

    let expanded = quote! {
        // Not for T
        impl #impl_generics ::std::ops::Not for #ident #ty_generics #where_clause {
            type Output = crate::logical_checks::operators::not::InvertedCheck<Self>;

            fn not(self) -> Self::Output {
                crate::logical_checks::operators::not::InvertedCheck::new(self)
            }
        }

        // BitAnd<Rhs> for T
        impl #impl_generics_rhs ::std::ops::BitAnd<Rhs> for #ident #ty_generics #where_clause_rhs {
            type Output = crate::logical_checks::operators::and::AndCheck<Self, Rhs>;

            fn bitand(self, rhs: Rhs) -> Self::Output {
                crate::logical_checks::operators::and::AndCheck::new(self, rhs)
            }
        }

        // BitOr<Rhs> for T
        impl #impl_generics_rhs ::std::ops::BitOr<Rhs> for #ident #ty_generics #where_clause_rhs {
            type Output = crate::logical_checks::operators::or::OrCheck<Self, Rhs>;

            fn bitor(self, rhs: Rhs) -> Self::Output {
                crate::logical_checks::operators::or::OrCheck::new(self, rhs)
            }
        }

        // From<T> for bool
        impl #impl_generics core::convert::From<#ident #ty_generics> for bool #where_clause {
            fn from(value: #ident #ty_generics) -> Self {
                value.check()
            }
        }

    };

    TokenStream::from(expanded)
}
