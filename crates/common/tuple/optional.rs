/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

use super::TupleUnoptional;

pub trait TupleOptional {
    type TupleOptional: Default + TupleUnoptional;
}

impl<T1> 
TupleOptional for (T1,) {
    type TupleOptional = (Option<T1>,); 
}

impl<T1, T2> 
TupleOptional for (T1,T2,) { 
    type TupleOptional = (Option<T1>, Option<T2>,); 
}

impl<T1, T2, T3>
TupleOptional for (T1,T2,T3,) {
    type TupleOptional = (Option<T1>, Option<T2>, Option<T3>,);
}

impl<T1, T2, T3, T4>
TupleOptional for (T1,T2,T3,T4,) { 
    type TupleOptional = (Option<T1>, Option<T2>, Option<T3>, Option<T4>,);
}

impl<T1, T2, T3, T4, T5> 
TupleOptional for (T1,T2,T3,T4,T5,) { 
    type TupleOptional = (Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>,); 
}

impl<T1, T2, T3, T4, T5, T6> 
TupleOptional for (T1,T2,T3,T4,T5,T6,) { 
    type TupleOptional = (Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>,); 
}

impl<T1, T2, T3, T4, T5, T6, T7>
TupleOptional for (T1,T2,T3,T4,T5,T6,T7,) {
    type TupleOptional = (Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>,); 
}

impl<T1, T2, T3, T4, T5, T6, T7, T8>
TupleOptional for (T1,T2,T3,T4,T5,T6,T7,T8,) {
    type TupleOptional = (Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>,); 
}

impl<T1, T2, T3, T4, T5, T6, T7, T8, T9>
TupleOptional for (T1,T2,T3,T4,T5,T6,T7,T8,T9,) { 
    type TupleOptional = (Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>,); 
}

impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> 
TupleOptional for (T1,T2,T3,T4,T5,T6,T7,T8,T9,T10) { 
    type TupleOptional = (Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>,); 
}

impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11>
TupleOptional for (T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11) {
    type TupleOptional = (Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>, Option<T11>,); 
}

impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12>
TupleOptional for (T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12) {
    type TupleOptional = (Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>, Option<T11>, Option<T12>); 
}