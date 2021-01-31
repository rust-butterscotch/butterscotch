/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

use crate::{ECS, EntityID};

// // Storage types // //

pub trait ReqRefComponents<'a> {
    fn retrieve(ecs: &'a ECS, eid: EntityID) -> Option<Self> where Self: Sized;
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

