/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

use arrayvec::ArrayVec;
use butterscotch_codegen::generate_tuple_impls;

use crate::{Component, ComponentID, ECS, EntityID, QueryID};

// // Storage types // //

pub trait ReqRefComponents<'a> {
    fn retrieve(ecs: &'a ECS, eid: EntityID) -> Option<Self> where Self: Sized;
    fn ids() -> ArrayVec<[ComponentID; 8]>;
}

pub trait OptRefComponents<'a> {
    fn retrieve(ecs: &'a ECS, eid: EntityID) -> Self;
}

// // User Definiition Helpers // //

pub trait ReqRefComponentsDefinition<'a> {
    type TupleType: ReqRefComponents<'a>;
}

pub trait OptRefComponentsDefinition<'a> {
    type TupleType: OptRefComponents<'a>;
}

pub trait ComponentRequestTupleDefinition<'a> {
    type ReqRefComponentTuple: ReqRefComponents<'a>;
    type OptRefComponentTuple: OptRefComponents<'a>;
}

impl<'a, T: ReqRefComponentsDefinition<'a>, U: OptRefComponentsDefinition<'a>> ComponentRequestTupleDefinition<'a> for (T,U,) {
    type ReqRefComponentTuple = T::TupleType;
    type OptRefComponentTuple = U::TupleType;
}

impl<'a, T: ReqRefComponentsDefinition<'a>> ComponentRequestTupleDefinition<'a> for (T,) {
    type ReqRefComponentTuple = T::TupleType;
    type OptRefComponentTuple = ();
}

// // Unit Tuple Impl // //

impl<'a> ReqRefComponents<'a> for () {
    fn retrieve(_ecs: &'a ECS, _eid: EntityID) -> Option<Self> where Self: Sized { Some(()) }
    fn ids() -> ArrayVec<[ComponentID; 8]> { Default::default() }
}

impl<'a> OptRefComponents<'a> for () {
    fn retrieve(_ecs: &'a ECS, _eid: EntityID) -> Self where Self: Sized { () }
}

impl<'a> ReqRefComponentsDefinition<'a> for () {
    type TupleType = ();
}

impl<'a> OptRefComponentsDefinition<'a> for () {
    type TupleType = ();
}

// // Impl Tuples // //

generate_tuple_impls!(8, r"
    impl<'a, %{%TR: Component,%}>
    ReqRefComponentsDefinition<'a> for (%{%TR, %}) {
        type TupleType = (%{&'a %TR, %});
    }

    impl<'a, %{%TR: Component,%}>
    ReqRefComponents<'a> for (%{&'a %TR, %}) {
        fn retrieve(ecs: &'a ECS, eid: EntityID) -> Option<Self> {Some((%{
            ecs.get_store_ref::<%TR>().get_ref(eid)?,%}
        ))}

        fn ids() -> QueryID {
            let mut result = QueryID::new();%{
            result.push(%TR::ID);%}
            result
        }
    }
");

generate_tuple_impls!(8, r"
    impl<'a, %{%TR: Component,%}>
    OptRefComponentsDefinition<'a> for (%{%TR, %}) {
        type TupleType = (%{Option<&'a %TR>, %});
    }

    impl<'a, %{%TR: Component,%}>
    OptRefComponents<'a> for (%{Option<&'a %TR>, %}) {
        fn retrieve(ecs: &'a ECS, eid: EntityID) -> Self {(%{
            ecs.get_store_ref::<%TR>().get_ref(eid),%}
        )}
    }
");