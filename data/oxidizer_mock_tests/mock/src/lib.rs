
// #[macro_export]
// macro_rules! mock_fn {
//     ($vis:vis fn $fn_name:ident ($($arg:ident:$typ:ty),*) $(-> $rt:ty)? $body:block) => {
//         $vis fn $fn_name($($arg:$typ,)*) $(-> $rt )? {
//             todo!()
//         }
        
//     };
// }

// #[macro_export]
// macro_rules! mock_body {
//     ( $body:block ) => {
//         todo!()
//     };
// }


extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
/// `mock_body` receives code inside a block, returning a block containing a single
/// `todo!()` statement
pub fn mock_body(_item: TokenStream) -> TokenStream {
    let output: proc_macro2::TokenStream = quote::quote!(
        todo!()
    );

    proc_macro::TokenStream::from(output)
}
