/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */
use butterscotch::ecs::{Component, ComponentID, ComponentRequestTupleDefinition, ECS, EntityID, OptRefComponents, ReqRefComponents};

#[derive(Debug, Component)]
#[component_ns = "Butterscotch"]
struct Component1 {}

#[derive(Debug, Component)]
#[component_ns = "Butterscotch"]
struct Component2 {}

#[derive(Debug, Component)]
#[component_ns = "Butterscotch"]
struct Component3 {}

#[derive(Debug, Component)]
#[component_ns = "Butterscotch"]
struct Component4 {}

fn main() {
    let mut ecs = ECS::default();
    ecs.register_component::<Component1>();
    ecs.register_component::<Component2>();
    ecs.register_component::<Component3>();
    ecs.register_component::<Component4>();
    if let Some(v) = call::<((Component1, Component2), ())>(&ecs, EntityID::new()) {
        println!("{:?}", v);
    }
}

fn call<'a, T: ComponentRequestTupleDefinition<'a>>(ecs: &'a ECS, eid: EntityID) -> Option<(T::ReqRefComponentTuple, T::OptRefComponentTuple)> {
    return Some((
        T::ReqRefComponentTuple::retrieve(ecs, eid)?,
        T::OptRefComponentTuple::retrieve(ecs, eid),
    ));
}
