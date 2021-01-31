/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

use super::TupleOptional;

pub trait TupleUnoptional {
    type TupleUnoptional: TupleOptional;
}

impl<T1>
TupleUnoptional for (Option<T1>,) {
    type TupleUnoptional = (T1,); 
}

impl<T1, T2>
TupleUnoptional for (Option<T1>, Option<T2>,) {
    type TupleUnoptional = (T1,T2,); 
}

impl<T1, T2, T3>
TupleUnoptional for (Option<T1>, Option<T2>, Option<T3>) {
    type TupleUnoptional = (T1,T2,T3,); 
}

impl<T1, T2, T3, T4>
TupleUnoptional for (Option<T1>, Option<T2>, Option<T3>, Option<T4>,) {
    type TupleUnoptional = (T1,T2,T3,T4,); 
}

impl<T1, T2, T3, T4, T5>
TupleUnoptional for (Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>,) {
    type TupleUnoptional = (T1,T2,T3,T4,T5,); 
}

impl<T1, T2, T3, T4, T5, T6>
TupleUnoptional for (Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>,) {
    type TupleUnoptional = (T1,T2,T3,T4,T5,T6,); 
}

impl<T1, T2, T3, T4, T5, T6, T7>
TupleUnoptional for (Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>,) {
    type TupleUnoptional = (T1,T2,T3,T4,T5,T6,T7,); 
}

impl<T1, T2, T3, T4, T5, T6, T7, T8>
TupleUnoptional for (Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>,) {
    type TupleUnoptional = (T1,T2,T3,T4,T5,T6,T7,T8,); 
}

impl<T1, T2, T3, T4, T5, T6, T7, T8, T9>
TupleUnoptional for (Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>,) {
    type TupleUnoptional = (T1,T2,T3,T4,T5,T6,T7,T8,T9,); 
}

impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10>
TupleUnoptional for (Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>,) {
    type TupleUnoptional = (T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,); 
}

impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11>
TupleUnoptional for (Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>, Option<T11>,) {
    type TupleUnoptional = (T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,); 
}

impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12>
TupleUnoptional for (Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>, Option<T11>, Option<T12>) {
    type TupleUnoptional = (T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12); 
}