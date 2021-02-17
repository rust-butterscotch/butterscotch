/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

use std::any::TypeId;

use butterscotch_codegen::generate_tuple_impls;

use crate::{container::ChunkyVec, tuple::{TupleElementGetter, TupleElementWrapper, TupleRef}, utility::{downcast_ref_unchecked, downcast_mut_unchecked}};



pub trait GIDMultiStoreHelper where Self: TupleElementWrapper + TupleRef, <Self as TupleElementWrapper>::WrapWith<ChunkyVec<()>>: TupleElementGetter {
    fn insert(self, vec: &mut <Self as TupleElementWrapper>::WrapWith<ChunkyVec<()>>, idx: usize);

    fn get<'a>(vec: &'a <Self as TupleElementWrapper>::WrapWith<ChunkyVec<()>>, idx: usize) -> Self::AsRef<'a>;

    fn get_store_ref<'a, T: 'static>(vec: &'a <Self as TupleElementWrapper>::WrapWith<ChunkyVec<()>>, idx: usize) -> &'a ChunkyVec<T>;
    fn get_store_mut<'a, T: 'static>(vec: &'a mut <Self as TupleElementWrapper>::WrapWith<ChunkyVec<()>>, idx: usize) -> &'a mut ChunkyVec<T>;

    fn swap_remove(vec: &mut <Self as TupleElementWrapper>::WrapWith<ChunkyVec<()>>, idx: usize) -> Self;
}

generate_tuple_impls!(8, r#"
    impl<%{%TR: 'static,%}> GIDMultiStoreHelper for (%{%TR,%}) {
        fn insert(self, vec: &mut <Self as TupleElementWrapper>::WrapWith<ChunkyVec<()>>, idx: usize) {
            let (%{v%VI,%}) = self;
            %{assert!(vec.get_%VI().len() == idx);%}
            %{vec.get_mut_%VI().push(v%VI);%}
        }
        
        fn get<'a>(vec: &'a <Self as TupleElementWrapper>::WrapWith<ChunkyVec<()>>, idx: usize) -> Self::AsRef<'a> {
            return (%{vec.get_%VI().get(idx).unwrap(),%});
        }
        
        fn get_store_ref<'a, T: 'static>(vec: &'a <Self as TupleElementWrapper>::WrapWith<ChunkyVec<()>>, idx: usize) -> &'a ChunkyVec<T> {
            let target = TypeId::of::<ChunkyVec<T>>();
            %{if target == TypeId::of::<%TR>() { return unsafe{downcast_ref_unchecked::<&ChunkyVec<T>>(&vec.%VI)}; } %}
            unreachable!("Type not in tuple!");
            //match target {
            //    %{TypeId::of::<%TR>() =>unsafe{downcast_ref_unchecked::<&ChunkyVec<T>>(&vec.%VI)}, %}
            //    _ => unreachable!("Type not in tuple!"),
            //}
        }
        
        fn get_store_mut<'a, T: 'static>(vec: &'a mut <Self as TupleElementWrapper>::WrapWith<ChunkyVec<()>>, idx: usize) -> &'a mut ChunkyVec<T> {
            let target = TypeId::of::<T>();
            %{if target == TypeId::of::<%TR>() { return unsafe{downcast_mut_unchecked::<&mut ChunkyVec<T>>(&mut vec.%VI)}; } %}
            unreachable!("Type not in tuple!");
        }

        fn swap_remove(vec: &mut <Self as TupleElementWrapper>::WrapWith<ChunkyVec<()>>, idx: usize) -> Self {
            return (%{vec.get_mut_%VI().swap_remove(idx),%});
        }
    }
"#);
