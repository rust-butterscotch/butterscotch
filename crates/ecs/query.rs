/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

use std::collections::{HashMap, HashSet};

use arrayvec::ArrayVec;
use butterscotch_common::container::GIDStore;

use crate::{ComponentID, EntityID, QueryID, ReqRefComponents, ReqRefComponentsDefinition};

#[derive(Debug, Default)]
pub(crate) struct QueryRef {
    pub uid: QueryID,
    pub idx: u32,
}

#[derive(Debug, Default)]
pub struct QueryContainer {
    queries: HashMap<QueryID, QueryData>
}

#[derive(Debug, Default)]
pub struct QueryData {
    target: u8,
    masks: GIDStore<u8>,
    entities: HashSet<EntityID>,
}

pub trait QueryUpdater {
    fn register_query(&mut self, query: (QueryID, u8));
}

impl QueryContainer {

    pub fn register<'a, T: ReqRefComponentsDefinition<'a>>(&mut self, updaters: &mut HashMap<ComponentID, Box<dyn QueryUpdater>>) -> QueryID {
        let mut ids = T::TupleType::ids();
        ids.sort_unstable(); // Force a reliable ordering
        let ids = ids;

        // Already have it? Skip.
        if self.queries.contains_key(&ids) { return ids; }

        // Check all component types accounted for, otherwise false
        for id in ids.iter() {
            if !updaters.contains_key(&id) { return ids; }
        }

        // Register query with all component types
        for i in 0..ids.len() {
            updaters.get_mut(&ids[i]).unwrap().register_query((ids.clone(), i as u8));
        }

        // Create query
        let mut data = QueryData::default();
        for _ in 0..ids.len() { data.target = data.target << 1 | 1; } // Create "complete" mask
        self.queries.insert(ids.clone(), data);

        return ids;
    }

    pub fn update_presence(&mut self, eid: EntityID, id: QueryID, index: u8, attached: bool) {
        assert!((index as usize) < id.len(), "Query index out of range.");
        match self.queries.get_mut(&id) {
            Some(v) => {
                let key = v.masks.get(eid).copied().unwrap_or(0);
                let key = if attached { key | (1 << index) } else { key & !(1 << index) };
                v.masks.replace(eid, key);
                if key == v.target {
                    v.entities.insert(eid);
                } else {
                    v.entities.remove(&eid);
                }
            },
            None => { panic!("Query not found!"); }
        }
    }

    pub fn query(&self, id: QueryID, destination: &mut Vec<EntityID>) {
        match self.queries.get(&id) {
            Some(v) => { destination.extend(v.entities.iter()) },
            None => { panic!("Query not found!"); }
        }
    }

}





