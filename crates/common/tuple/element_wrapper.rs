/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

use butterscotch_codegen::generate_tuple_impls;
use crate::utility::GenericRetype;

pub trait TupleElementWrapper {
    type WrapWith<T: GenericRetype>;
}

generate_tuple_impls!(8, r"
    impl<%{%TR,%}> TupleElementWrapper for (%{%TR,%}) {
        type WrapWith<T: GenericRetype> = (%{T::RetypeWith<%TR>,%});
    }
");