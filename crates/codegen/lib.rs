/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

use proc_macro::TokenStream;

mod tuple_impl;
use tuple_impl::*;

#[proc_macro]
pub fn generate_tuple_impls(input: TokenStream) -> TokenStream {
    do_generate_tuple_impls(input)
}