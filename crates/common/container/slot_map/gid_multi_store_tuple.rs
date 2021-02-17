/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

use butterscotch_codegen::generate_tuple_impls;

use crate::{tuple::{TupleElementGetter, TupleElementWrapper, TupleRef}, container::ChunkyVec};

pub trait MultiStoreIO where Self: TupleElementWrapper + TupleRef, <Self as TupleElementWrapper>::WrapWith<ChunkyVec<()>>: TupleElementGetter {
    fn insert(self, vec: &mut <Self as TupleElementWrapper>::WrapWith<ChunkyVec<()>>, idx: usize);

    fn get<'a>(vec: &'a <Self as TupleElementWrapper>::WrapWith<ChunkyVec<()>>, idx: usize) -> Self::AsRef<'a>;

    fn swap_remove(vec: &mut <Self as TupleElementWrapper>::WrapWith<ChunkyVec<()>>, idx: usize) -> Self;

}

generate_tuple_impls!(8, r"
    impl<%{%TR: 'static,%}> MultiStoreIO for (%{%TR,%}) {
        fn insert(self, vec: &mut <Self as TupleElementWrapper>::WrapWith<ChunkyVec<()>>, idx: usize) {
            let (%{v%VI,%}) = self;
            %{assert!(vec.get_%VI().len() == idx);%}
            %{vec.get_mut_%VI().push(v%VI);%}
        }
        
        fn get<'a>(vec: &'a <Self as TupleElementWrapper>::WrapWith<ChunkyVec<()>>, idx: usize) -> Self::AsRef<'a> {
            return (%{vec.get_%VI().get(idx).unwrap(),%});
        }

        fn swap_remove(vec: &mut <Self as TupleElementWrapper>::WrapWith<ChunkyVec<()>>, idx: usize) -> Self {
            return (%{vec.get_mut_%VI().swap_remove(idx),%});
        }
    }
");