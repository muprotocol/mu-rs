use quote::quote;
use std::fs;
use std::path::Path;

#[proc_macro_attribute]
pub fn function(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item = syn::parse_macro_input!(item as syn::Item);

    let build_uuid = std::env::var("MU_BUILD_UUID").unwrap();
    let out_dir = std::env::var("OUT_DIR").unwrap();

    let add_export_candid = !Path::new(&format!("{}/{}", out_dir, build_uuid)).exists();
    fs::write(format!("{}/{}", out_dir, build_uuid), "").unwrap();

    let r = quote! {
        #[ic_cdk::update]
        #item
    };

    if add_export_candid || true {
        quote! {
            #r
            ic_cdk::export_candid!();
        }
    } else {
        r
    }
    .into()
}
