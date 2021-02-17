/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

use butterscotch_codegen::generate_tuple_impls;

pub trait TupleRef {
    type AsRef<'a>;
    type AsMut<'a>;
}

generate_tuple_impls!(8, r"
    impl<%{%TR: 'static,%}> TupleRef for (%{%TR,%}) {
        type AsRef<'a> = (%{&'a %TR,%});
        type AsMut<'a> = (%{&'a mut %TR,%});
    }
");