use crate::{Component, ComponentRequestTupleDefinition, ECS, EntityID, OptRefComponents, ReqRefComponents};

#[derive(Debug)]
struct Component1 {}
impl Component for Component1 {}

#[derive(Debug)]
struct Component2 {}
impl Component for Component2 {}

#[derive(Debug)]
struct Component3 {}
impl Component for Component3 {}

#[derive(Debug)]
struct Component4 {}
impl Component for Component4 {}

#[test]
fn test() {
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