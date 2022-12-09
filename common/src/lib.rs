extern crate core;

use proc_macro::quote;
use std::any::Any;

use syn::{DeriveInput, parse_macro_input};

mod runner;
mod net;

#[cfg(test)]
mod tests {
    use std::any::Any;
    use syn::parse_macro_input;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

/**
 * 反射识别String类型，如果是则为其加_change
 * 如果不是则啥也不干
 */
trait AddMark{

    fn mark(&self){
        core::any::

    }
}

pub fn do_mark(s: &dyn Any) {
    let is_string = s.is::<String>();
    if is_string {
        let mut string = s.downcast_ref::<String>().unwrap();
        string.push_str("_change");
    }
}


/**
 * 定义一个派生宏
 */
#[proc_macro_derive(Trait)]
pub fn my_derive_trait(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    //这似乎是一个类似java强转的宏
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let expended = quote! {
        impl Trait for #name {

        }
    };

    return proc_macro::TokenStream::from(expended);
}
