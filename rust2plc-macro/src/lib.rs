use proc_macro::TokenStream;
use rust2plc::langs::PLCLang;

#[proc_macro_attribute]
pub fn plc_fn(attr: TokenStream, item: TokenStream) -> TokenStream {
    eprintln!("plc_fn macro called with attr: {:?}", attr);
    
    // let dialect: PLCLang = attr.to_string().as_str().into();

    item
}
