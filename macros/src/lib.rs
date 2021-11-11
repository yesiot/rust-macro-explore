use syn;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;

#[proc_macro_attribute]
pub fn show_streams(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("Inspection:");
    println!("attr: \"{}\"", attr.to_string());
    println!("item: \"{}\"", item.to_string());
    item
}

#[proc_macro_attribute]
pub fn reverse_name(attr: TokenStream, item: TokenStream) -> TokenStream {

    // turn TokenStream into a syntax tree
    let func = syn::parse_macro_input!(item as syn::ItemFn);

    // extract fields out of the item
    let syn::ItemFn {
        attrs,
        vis,
        mut sig,    // we are going to change the signature
        block,
    } = func;

    let name = (format!("{}", sig.ident)).chars().rev().collect::<String>();
    sig.ident = syn::Ident::new(&name, Span::call_site());

    let item_str = attr.to_string();

    let output = quote! {
        #(#attrs)*
        #vis #sig {
            println!("Injected: {}", #item_str);
            #block
            println!("Again injected: {}", #item_str);
        }
    };

    //See the body of our new function (printed during build)
    println!("New function:\n{}", output.to_string());

    // Convert the output from a `proc_macro2::TokenStream` to a `proc_macro::TokenStream`
    TokenStream::from(output)
}