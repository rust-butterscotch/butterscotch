
/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** *      GENERATED FILE, DO NOT EDIT, IT WILL BE REGENERATED ON BUILD    * **
** ************************************************************************ */

use crate::{Component, ECS, EntityID, ReqRefComponents, OptRefComponents, ReqRefComponentsDefinition, OptRefComponentsDefinition, QueryID};

// // ReqRefComponents // //
impl<'a, T0: Component,>
ReqRefComponentsDefinition<'a> for (T0, ) {
    type TupleType = (&'a T0, );
}

impl<'a, T0: Component,T1: Component,>
ReqRefComponentsDefinition<'a> for (T0, T1, ) {
    type TupleType = (&'a T0, &'a T1, );
}

impl<'a, T0: Component,T1: Component,T2: Component,>
ReqRefComponentsDefinition<'a> for (T0, T1, T2, ) {
    type TupleType = (&'a T0, &'a T1, &'a T2, );
}

impl<'a, T0: Component,T1: Component,T2: Component,T3: Component,>
ReqRefComponentsDefinition<'a> for (T0, T1, T2, T3, ) {
    type TupleType = (&'a T0, &'a T1, &'a T2, &'a T3, );
}

impl<'a, T0: Component,T1: Component,T2: Component,T3: Component,T4: Component,>
ReqRefComponentsDefinition<'a> for (T0, T1, T2, T3, T4, ) {
    type TupleType = (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, );
}

impl<'a, T0: Component,T1: Component,T2: Component,T3: Component,T4: Component,T5: Component,>
ReqRefComponentsDefinition<'a> for (T0, T1, T2, T3, T4, T5, ) {
    type TupleType = (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, );
}

impl<'a, T0: Component,T1: Component,T2: Component,T3: Component,T4: Component,T5: Component,T6: Component,>
ReqRefComponentsDefinition<'a> for (T0, T1, T2, T3, T4, T5, T6, ) {
    type TupleType = (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, );
}

impl<'a, T0: Component,T1: Component,T2: Component,T3: Component,T4: Component,T5: Component,T6: Component,T7: Component,>
ReqRefComponentsDefinition<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, ) {
    type TupleType = (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, );
}

impl<'a, T0: Component,>
ReqRefComponents<'a> for (&'a T0, ) {
    fn retrieve(ecs: &'a ECS, eid: EntityID) -> Option<Self> {Some((
        ecs.get_store_ref::<T0>().get_ref(eid)?,
    ))}

    fn ids() -> QueryID {
        let mut result = QueryID::new();
        result.push(T0::ID);
        result
    }
}

impl<'a, T0: Component,T1: Component,>
ReqRefComponents<'a> for (&'a T0, &'a T1, ) {
    fn retrieve(ecs: &'a ECS, eid: EntityID) -> Option<Self> {Some((
        ecs.get_store_ref::<T0>().get_ref(eid)?,
        ecs.get_store_ref::<T1>().get_ref(eid)?,
    ))}

    fn ids() -> QueryID {
        let mut result = QueryID::new();
        result.push(T0::ID);
        result.push(T1::ID);
        result
    }
}

impl<'a, T0: Component,T1: Component,T2: Component,>
ReqRefComponents<'a> for (&'a T0, &'a T1, &'a T2, ) {
    fn retrieve(ecs: &'a ECS, eid: EntityID) -> Option<Self> {Some((
        ecs.get_store_ref::<T0>().get_ref(eid)?,
        ecs.get_store_ref::<T1>().get_ref(eid)?,
        ecs.get_store_ref::<T2>().get_ref(eid)?,
    ))}

    fn ids() -> QueryID {
        let mut result = QueryID::new();
        result.push(T0::ID);
        result.push(T1::ID);
        result.push(T2::ID);
        result
    }
}

impl<'a, T0: Component,T1: Component,T2: Component,T3: Component,>
ReqRefComponents<'a> for (&'a T0, &'a T1, &'a T2, &'a T3, ) {
    fn retrieve(ecs: &'a ECS, eid: EntityID) -> Option<Self> {Some((
        ecs.get_store_ref::<T0>().get_ref(eid)?,
        ecs.get_store_ref::<T1>().get_ref(eid)?,
        ecs.get_store_ref::<T2>().get_ref(eid)?,
        ecs.get_store_ref::<T3>().get_ref(eid)?,
    ))}

    fn ids() -> QueryID {
        let mut result = QueryID::new();
        result.push(T0::ID);
        result.push(T1::ID);
        result.push(T2::ID);
        result.push(T3::ID);
        result
    }
}

impl<'a, T0: Component,T1: Component,T2: Component,T3: Component,T4: Component,>
ReqRefComponents<'a> for (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, ) {
    fn retrieve(ecs: &'a ECS, eid: EntityID) -> Option<Self> {Some((
        ecs.get_store_ref::<T0>().get_ref(eid)?,
        ecs.get_store_ref::<T1>().get_ref(eid)?,
        ecs.get_store_ref::<T2>().get_ref(eid)?,
        ecs.get_store_ref::<T3>().get_ref(eid)?,
        ecs.get_store_ref::<T4>().get_ref(eid)?,
    ))}

    fn ids() -> QueryID {
        let mut result = QueryID::new();
        result.push(T0::ID);
        result.push(T1::ID);
        result.push(T2::ID);
        result.push(T3::ID);
        result.push(T4::ID);
        result
    }
}

impl<'a, T0: Component,T1: Component,T2: Component,T3: Component,T4: Component,T5: Component,>
ReqRefComponents<'a> for (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, ) {
    fn retrieve(ecs: &'a ECS, eid: EntityID) -> Option<Self> {Some((
        ecs.get_store_ref::<T0>().get_ref(eid)?,
        ecs.get_store_ref::<T1>().get_ref(eid)?,
        ecs.get_store_ref::<T2>().get_ref(eid)?,
        ecs.get_store_ref::<T3>().get_ref(eid)?,
        ecs.get_store_ref::<T4>().get_ref(eid)?,
        ecs.get_store_ref::<T5>().get_ref(eid)?,
    ))}

    fn ids() -> QueryID {
        let mut result = QueryID::new();
        result.push(T0::ID);
        result.push(T1::ID);
        result.push(T2::ID);
        result.push(T3::ID);
        result.push(T4::ID);
        result.push(T5::ID);
        result
    }
}

impl<'a, T0: Component,T1: Component,T2: Component,T3: Component,T4: Component,T5: Component,T6: Component,>
ReqRefComponents<'a> for (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, ) {
    fn retrieve(ecs: &'a ECS, eid: EntityID) -> Option<Self> {Some((
        ecs.get_store_ref::<T0>().get_ref(eid)?,
        ecs.get_store_ref::<T1>().get_ref(eid)?,
        ecs.get_store_ref::<T2>().get_ref(eid)?,
        ecs.get_store_ref::<T3>().get_ref(eid)?,
        ecs.get_store_ref::<T4>().get_ref(eid)?,
        ecs.get_store_ref::<T5>().get_ref(eid)?,
        ecs.get_store_ref::<T6>().get_ref(eid)?,
    ))}

    fn ids() -> QueryID {
        let mut result = QueryID::new();
        result.push(T0::ID);
        result.push(T1::ID);
        result.push(T2::ID);
        result.push(T3::ID);
        result.push(T4::ID);
        result.push(T5::ID);
        result.push(T6::ID);
        result
    }
}

impl<'a, T0: Component,T1: Component,T2: Component,T3: Component,T4: Component,T5: Component,T6: Component,T7: Component,>
ReqRefComponents<'a> for (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, ) {
    fn retrieve(ecs: &'a ECS, eid: EntityID) -> Option<Self> {Some((
        ecs.get_store_ref::<T0>().get_ref(eid)?,
        ecs.get_store_ref::<T1>().get_ref(eid)?,
        ecs.get_store_ref::<T2>().get_ref(eid)?,
        ecs.get_store_ref::<T3>().get_ref(eid)?,
        ecs.get_store_ref::<T4>().get_ref(eid)?,
        ecs.get_store_ref::<T5>().get_ref(eid)?,
        ecs.get_store_ref::<T6>().get_ref(eid)?,
        ecs.get_store_ref::<T7>().get_ref(eid)?,
    ))}

    fn ids() -> QueryID {
        let mut result = QueryID::new();
        result.push(T0::ID);
        result.push(T1::ID);
        result.push(T2::ID);
        result.push(T3::ID);
        result.push(T4::ID);
        result.push(T5::ID);
        result.push(T6::ID);
        result.push(T7::ID);
        result
    }
}
// // OptRefComponents // //
impl<'a, T0: Component,>
OptRefComponentsDefinition<'a> for (T0, ) {
    type TupleType = (Option<&'a T0>, );
}

impl<'a, T0: Component,T1: Component,>
OptRefComponentsDefinition<'a> for (T0, T1, ) {
    type TupleType = (Option<&'a T0>, Option<&'a T1>, );
}

impl<'a, T0: Component,T1: Component,T2: Component,>
OptRefComponentsDefinition<'a> for (T0, T1, T2, ) {
    type TupleType = (Option<&'a T0>, Option<&'a T1>, Option<&'a T2>, );
}

impl<'a, T0: Component,T1: Component,T2: Component,T3: Component,>
OptRefComponentsDefinition<'a> for (T0, T1, T2, T3, ) {
    type TupleType = (Option<&'a T0>, Option<&'a T1>, Option<&'a T2>, Option<&'a T3>, );
}

impl<'a, T0: Component,T1: Component,T2: Component,T3: Component,T4: Component,>
OptRefComponentsDefinition<'a> for (T0, T1, T2, T3, T4, ) {
    type TupleType = (Option<&'a T0>, Option<&'a T1>, Option<&'a T2>, Option<&'a T3>, Option<&'a T4>, );
}

impl<'a, T0: Component,T1: Component,T2: Component,T3: Component,T4: Component,T5: Component,>
OptRefComponentsDefinition<'a> for (T0, T1, T2, T3, T4, T5, ) {
    type TupleType = (Option<&'a T0>, Option<&'a T1>, Option<&'a T2>, Option<&'a T3>, Option<&'a T4>, Option<&'a T5>, );
}

impl<'a, T0: Component,T1: Component,T2: Component,T3: Component,T4: Component,T5: Component,T6: Component,>
OptRefComponentsDefinition<'a> for (T0, T1, T2, T3, T4, T5, T6, ) {
    type TupleType = (Option<&'a T0>, Option<&'a T1>, Option<&'a T2>, Option<&'a T3>, Option<&'a T4>, Option<&'a T5>, Option<&'a T6>, );
}

impl<'a, T0: Component,T1: Component,T2: Component,T3: Component,T4: Component,T5: Component,T6: Component,T7: Component,>
OptRefComponentsDefinition<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, ) {
    type TupleType = (Option<&'a T0>, Option<&'a T1>, Option<&'a T2>, Option<&'a T3>, Option<&'a T4>, Option<&'a T5>, Option<&'a T6>, Option<&'a T7>, );
}

impl<'a, T0: Component,>
OptRefComponents<'a> for (Option<&'a T0>, ) {
    fn retrieve(ecs: &'a ECS, eid: EntityID) -> Self {(
        ecs.get_store_ref::<T0>().get_ref(eid),
    )}
}

impl<'a, T0: Component,T1: Component,>
OptRefComponents<'a> for (Option<&'a T0>, Option<&'a T1>, ) {
    fn retrieve(ecs: &'a ECS, eid: EntityID) -> Self {(
        ecs.get_store_ref::<T0>().get_ref(eid),
        ecs.get_store_ref::<T1>().get_ref(eid),
    )}
}

impl<'a, T0: Component,T1: Component,T2: Component,>
OptRefComponents<'a> for (Option<&'a T0>, Option<&'a T1>, Option<&'a T2>, ) {
    fn retrieve(ecs: &'a ECS, eid: EntityID) -> Self {(
        ecs.get_store_ref::<T0>().get_ref(eid),
        ecs.get_store_ref::<T1>().get_ref(eid),
        ecs.get_store_ref::<T2>().get_ref(eid),
    )}
}

impl<'a, T0: Component,T1: Component,T2: Component,T3: Component,>
OptRefComponents<'a> for (Option<&'a T0>, Option<&'a T1>, Option<&'a T2>, Option<&'a T3>, ) {
    fn retrieve(ecs: &'a ECS, eid: EntityID) -> Self {(
        ecs.get_store_ref::<T0>().get_ref(eid),
        ecs.get_store_ref::<T1>().get_ref(eid),
        ecs.get_store_ref::<T2>().get_ref(eid),
        ecs.get_store_ref::<T3>().get_ref(eid),
    )}
}

impl<'a, T0: Component,T1: Component,T2: Component,T3: Component,T4: Component,>
OptRefComponents<'a> for (Option<&'a T0>, Option<&'a T1>, Option<&'a T2>, Option<&'a T3>, Option<&'a T4>, ) {
    fn retrieve(ecs: &'a ECS, eid: EntityID) -> Self {(
        ecs.get_store_ref::<T0>().get_ref(eid),
        ecs.get_store_ref::<T1>().get_ref(eid),
        ecs.get_store_ref::<T2>().get_ref(eid),
        ecs.get_store_ref::<T3>().get_ref(eid),
        ecs.get_store_ref::<T4>().get_ref(eid),
    )}
}

impl<'a, T0: Component,T1: Component,T2: Component,T3: Component,T4: Component,T5: Component,>
OptRefComponents<'a> for (Option<&'a T0>, Option<&'a T1>, Option<&'a T2>, Option<&'a T3>, Option<&'a T4>, Option<&'a T5>, ) {
    fn retrieve(ecs: &'a ECS, eid: EntityID) -> Self {(
        ecs.get_store_ref::<T0>().get_ref(eid),
        ecs.get_store_ref::<T1>().get_ref(eid),
        ecs.get_store_ref::<T2>().get_ref(eid),
        ecs.get_store_ref::<T3>().get_ref(eid),
        ecs.get_store_ref::<T4>().get_ref(eid),
        ecs.get_store_ref::<T5>().get_ref(eid),
    )}
}

impl<'a, T0: Component,T1: Component,T2: Component,T3: Component,T4: Component,T5: Component,T6: Component,>
OptRefComponents<'a> for (Option<&'a T0>, Option<&'a T1>, Option<&'a T2>, Option<&'a T3>, Option<&'a T4>, Option<&'a T5>, Option<&'a T6>, ) {
    fn retrieve(ecs: &'a ECS, eid: EntityID) -> Self {(
        ecs.get_store_ref::<T0>().get_ref(eid),
        ecs.get_store_ref::<T1>().get_ref(eid),
        ecs.get_store_ref::<T2>().get_ref(eid),
        ecs.get_store_ref::<T3>().get_ref(eid),
        ecs.get_store_ref::<T4>().get_ref(eid),
        ecs.get_store_ref::<T5>().get_ref(eid),
        ecs.get_store_ref::<T6>().get_ref(eid),
    )}
}

impl<'a, T0: Component,T1: Component,T2: Component,T3: Component,T4: Component,T5: Component,T6: Component,T7: Component,>
OptRefComponents<'a> for (Option<&'a T0>, Option<&'a T1>, Option<&'a T2>, Option<&'a T3>, Option<&'a T4>, Option<&'a T5>, Option<&'a T6>, Option<&'a T7>, ) {
    fn retrieve(ecs: &'a ECS, eid: EntityID) -> Self {(
        ecs.get_store_ref::<T0>().get_ref(eid),
        ecs.get_store_ref::<T1>().get_ref(eid),
        ecs.get_store_ref::<T2>().get_ref(eid),
        ecs.get_store_ref::<T3>().get_ref(eid),
        ecs.get_store_ref::<T4>().get_ref(eid),
        ecs.get_store_ref::<T5>().get_ref(eid),
        ecs.get_store_ref::<T6>().get_ref(eid),
        ecs.get_store_ref::<T7>().get_ref(eid),
    )}
}
