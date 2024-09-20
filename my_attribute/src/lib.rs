extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn simple_attribute(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input tokens as a function
    let input_fn = parse_macro_input!(item as ItemFn);

    // Get the original function name and body
    let fn_name = &input_fn.sig.ident;
    let fn_body = &input_fn.block; // Capture the original function body

    // Create a new function name for the inner function
    let inner_fn_name = syn::Ident::new(&format!("{}_inner", fn_name), fn_name.span());

    // Generate the new function with logging
    let output = quote! {
        pub async fn #fn_name(Json(payload): Json<UserData>) -> impl IntoResponse {
            log::info!("Entering {}", stringify!(#fn_name));

            let result = #inner_fn_name(payload).await;

            log::info!("Exiting {}", stringify!(#fn_name));
            result
        }

        async fn #inner_fn_name(payload: UserData) -> impl IntoResponse {
            // The original function body
            #fn_body
        }
    };

    output.into()
}
