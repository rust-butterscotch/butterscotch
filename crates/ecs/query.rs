/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

use std::collections::{HashMap, HashSet};

use butterscotch_common::container::{ChunkSize, GIDStore};

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

#[derive(Debug)]
pub struct QueryData {
    length: i8,
    masks: GIDStore<i8>,
    entities: HashSet<EntityID>,
}

impl QueryData {
    pub fn new(chunk_size: ChunkSize) -> Self {
        Self{
            length: 0,
            masks: GIDStore::new(chunk_size),
            entities: Default::default() // TODO quick hasher function
        }
    }
}

pub trait QueryUpdater {
    fn register_query(&mut self, query: (QueryID, u8));


    fn get_count_hint(&self) -> Option<usize>;
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

        let mut size_hint: Option<usize> = None;

        // Register query with all component types
        for i in 0..ids.len() {
            let query_updater = updaters.get_mut(&ids[i]).unwrap();
            query_updater.register_query((ids.clone(), i as u8));
            match query_updater.get_count_hint() {
                Some(v) => size_hint = Some(size_hint.unwrap_or(0).max(v)),
                None    => {},
            }
        }

        // Create query
        let mut data = QueryData::new(ChunkSize::Elements(size_hint.unwrap_or(1024)));
        debug_assert!(ids.len() <= std::i8::MAX as usize, "Mask overflow");
        data.length = ids.len() as i8;
        self.queries.insert(ids.clone(), data);

        return ids;
    }

    pub fn update_presence(&mut self, eid: EntityID, id: QueryID, index: u8, attach: bool) {
        assert!((index as usize) < id.len(), "Query index out of range.");
        match self.queries.get_mut(&id) {
            Some(v) => {
                let mask_old = v.masks.get(eid).copied().unwrap_or(0);

                // Modify mask
                let mask_new = mask_old + (if attach { 1 } else  { -1 });
                debug_assert!(mask_new >= 0,        "Mask dropped below zero"       );
                debug_assert!(mask_new <= v.length, "Mask raised above query length");
                v.masks.replace(eid, mask_new);

                // Update entities
                if mask_new == v.length {
                    if mask_old < v.length { 
                        v.entities.insert(eid); 
                    }
                } else if mask_old >= v.length { 
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





