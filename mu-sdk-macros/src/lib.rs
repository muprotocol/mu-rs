use quote::quote;
use syn::ItemFn;

#[proc_macro_attribute]
pub fn public(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let module = syn::parse_macro_input!(item as syn::ItemMod);

    let mod_ident = &module.ident; // Module name
    let mut new_content = Vec::new();

    // Iterate over all items in the module
    if let Some((_, items)) = &module.content {
        for item in items {
            if let syn::Item::Fn(func) = item {
                // Process each function
                new_content.push(process_function(func));
            } else {
                // Keep non-function items as they are
                new_content.push(quote! { #item });
            }
        }
    }

    // Reconstruct the module
    quote! {
        mod #mod_ident {
            #( #new_content )*
        }

        ic_cdk::export_candid!();
    }
    .into()
}

fn process_function(func: &ItemFn) -> proc_macro2::TokenStream {
    let mut transformed_func = func.clone();

    let has_function_attr = func
        .attrs
        .iter()
        .any(|attr| attr.path().is_ident("function"));
    if has_function_attr {
        // add the ic_cdk attribute to the function and remove the function attribute
        transformed_func
            .attrs
            .push(syn::parse_quote!(#[ic_cdk::update]));
        transformed_func
            .attrs
            .retain(|attr| !attr.path().is_ident("function"));
    }

    // Convert the function back into a token stream
    quote! {
        #transformed_func
    }
}
