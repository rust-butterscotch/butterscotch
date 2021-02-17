/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

use proc_macro::TokenStream;
use syn::{parse_macro_input, Token, ExprLit, Lit, parse::{Parse, ParseStream, Result}};

mod gen_impl;
use gen_impl::*;

struct TupleImplInput {
    templates: Vec<(usize, String)>,
}

impl Parse for TupleImplInput {
    fn parse(input: ParseStream) -> Result<Self> {
        // TODO this is a little gross...
        let args = input.parse_terminated::<ExprLit, Token![,]>(ExprLit::parse)?;
        let args = args.iter().collect::<Vec<&ExprLit>>();
        let args = args.chunks_exact(2);
        
        let templates: Vec<(usize, String)> = args.map(|v| (
            match &v[0].lit {
                Lit::Int(v) => v.base10_parse().unwrap(),
                _ => panic!("Expected integer count specifier")
            },
            match &v[1].lit {
                Lit::Str(v) => v.value(),
                _ => panic!("Expected template specifier")
            }
        )).collect();

        return Ok(Self{ templates })
    }
}

pub fn do_generate_tuple_impls(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as TupleImplInput);
    input.templates.iter()
        .map(|(count, template)| process_tuple_string(*count, template))
        .fold(TokenStream::new(), |mut v, n| { v.extend::<TokenStream>(n.parse().unwrap()); v })
}