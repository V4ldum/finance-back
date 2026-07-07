use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Ident};

/// Derives the full HTTP-error plumbing shared by every endpoint error.
///
/// Generates the three impls that are otherwise written by hand for each type:
/// - `ApiErrorResponse`, whose `status()` maps each variant to a `StatusCode`
///   named by its `#[status(CODE)]` tag. Every variant must carry a tag; a
///   missing tag is a compile error. `reason()` keeps the trait's default.
/// - `IntoResponse`, delegating to `response(&self)`.
/// - `Debug`, delegating to `error_chain_fmt`.
///
/// `Display`/`Error` still come from `thiserror::Error`.
///
/// The generated code references `crate::ApiErrorResponse`, `crate::response`,
/// and `crate::error_chain_fmt`, which the consuming crate re-exports at its
/// root.
///
/// ```ignore
/// #[derive(thiserror::Error, api_error_derive::ApiError)]
/// pub(crate) enum GetCoinError {
///     #[error("The provided id is invalid")]
///     #[status(NOT_FOUND)]
///     InvalidId,
///     #[error(transparent)]
///     #[status(INTERNAL_SERVER_ERROR)]
///     UnexpectedError(#[from] anyhow::Error),
/// }
/// ```
#[proc_macro_derive(ApiError, attributes(status))]
pub fn derive_api_error(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let Data::Enum(data) = &input.data else {
        return syn::Error::new_spanned(&input, "ApiError can only be derived for enums")
            .to_compile_error()
            .into();
    };

    let mut arms = Vec::new();
    for variant in &data.variants {
        let vname = &variant.ident;

        let mut status: Option<Ident> = None;
        for attr in &variant.attrs {
            if attr.path().is_ident("status") {
                match attr.parse_args::<Ident>() {
                    Ok(ident) => status = Some(ident),
                    Err(e) => return e.to_compile_error().into(),
                }
            }
        }
        let Some(status) = status else {
            return syn::Error::new_spanned(variant, "every variant needs a #[status(...)] tag")
                .to_compile_error()
                .into();
        };

        let pattern = match &variant.fields {
            Fields::Unit => quote! { #name::#vname },
            Fields::Unnamed(_) => quote! { #name::#vname(..) },
            Fields::Named(_) => quote! { #name::#vname { .. } },
        };

        arms.push(quote! {
            #pattern => ::axum::http::StatusCode::#status,
        });
    }

    let expanded = quote! {
        impl crate::ApiErrorResponse for #name {
            fn status(&self) -> ::axum::http::StatusCode {
                match self {
                    #(#arms)*
                }
            }
        }

        impl ::axum::response::IntoResponse for #name {
            fn into_response(self) -> ::axum::response::Response {
                crate::response(&self)
            }
        }

        impl ::std::fmt::Debug for #name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                crate::error_chain_fmt(self, f)
            }
        }
    };

    expanded.into()
}
