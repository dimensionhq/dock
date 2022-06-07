use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn command(attr: TokenStream, item: TokenStream) -> TokenStream {
    for attribute in attr.into_iter() {
        println!("Attribute: {}", attribute);
    }

    item
}
