/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */
use butterscotch_codegen::tuple_impl::*;

fn main() {
    std::fs::write("component_tuple_gen.rs", join(&[r"
/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** *      GENERATED FILE, DO NOT EDIT, IT WILL BE REGENERATED ON BUILD    * **
** ************************************************************************ */
use crate::{Component, ECS, EntityID, ReqRefComponents, OptRefComponents, ReqRefComponentsDefinition, OptRefComponentsDefinition};
",

"// // ReqRefComponents // //",
&process_tuple_string::<8>(r"
impl<'a, %{%TR: Component,%}>
ReqRefComponentsDefinition<'a> for (%{%TR, %}) {
    type TupleType = (%{&'a %TR, %});
}
"),&process_tuple_string::<8>(r"
impl<'a, %{%TR: Component,%}>
ReqRefComponents<'a> for (%{&'a %TR, %}) {
    fn retrieve(ecs: &'a ECS, eid: EntityID) -> Option<Self> {Some((%{
        ecs.get_store_ref::<%TR>().get_ref(eid)?,%}
    ))}
}
"),

"// // OptRefComponents // //",
&process_tuple_string::<8>(r"
impl<'a, %{%TR: Component,%}>
OptRefComponentsDefinition<'a> for (%{%TR, %}) {
    type TupleType = (%{Option<&'a %TR>, %});
}
"),&process_tuple_string::<8>(r"
impl<'a, %{%TR: Component,%}>
OptRefComponents<'a> for (%{Option<&'a %TR>, %}) {
    fn retrieve(ecs: &'a ECS, eid: EntityID) -> Self {(%{
        ecs.get_store_ref::<%TR>().get_ref(eid),%}
    )}
}
")])).expect("Failed to write generated code out");
}

fn join(a: &[&str]) -> String {
    let mut result = "".to_owned();
    for b in a {
        result.push_str(b);
    }
    return result;
}

// where %{%TR: ComponentMetadata<ComponentType = %TR>, <%TR as ComponentMetadata>::ComponentStoreType: ComponentMetadata<ComponentType = %TR>,%}


/*

impl<%{%TR: Component,%}>
ComponentTupleDef for (%{%VAR[(%TR)|(Option<%TR>)], %}) {
    type Head = T0;
    type ComponentTupleType<'a> = (%{%VAR[(&'a %TR)|(Option<&'a %TR>)], %});
}

impl<'a, %{%TR: Component,%}>
ComponentTuple<'a> for (%{%VAR[(&'a %TR)|(Option<&'a %TR>)], %}) {
    type Head = T0;
    fn try_retrieve(ecs: &'a ECS, eid: EntityID) -> Option<Self> {Some((%{
        ecs.get_store_ref::<%TR>().get_raw_ref(eid)%VAR[(?)|()],%}
    ))}
}


*/