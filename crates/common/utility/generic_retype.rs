use std::{vec::Vec, collections::VecDeque};

pub trait GenericRetype {
    type RetypeWith<T>: GenericRetype;
}

impl<T> GenericRetype for Vec<T> {
    type RetypeWith<R> = Vec<R>;
}

impl<T> GenericRetype for VecDeque<T> {
    type RetypeWith<R> = VecDeque<R>;
}

impl<T> GenericRetype for Option<T> {
    type RetypeWith<R> = Option<R>;
}