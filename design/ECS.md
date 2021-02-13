# ECS

## Redesign

### Old Version

Components in the old version were stored seperately
in a dedicated manager for the component. As a game
is played and entities are created/destroyed and 
components are added/removed, entities between these
stores become fragemented - which is bad for cache.

### New Version

Components in the new version are stored in entity groups
that share all common components. This is commonly known
as archetypes. This comes with some big upsides:
 - Queries don't require tracking for acceleration, archetype blocks can simply be iterated over
 - Component storage doesn't become fragmented, and can make more use of the cache

Some cons:
 - Arbitrary component lookup for an entity id is more costly without some kind of encoding scheme, that prohibits changing archetypes.
 - Attaching/Removing components is much more expensive as you need to move the entire entity

Some notes:
 - Our archetype system will utlize a arrays of components over an array of component tuples.
   - This is similar to the SoA vs AoS issue, but this rare instance SoA is as difficult to implement as AoS here AND makes sense for cache access as queries might not want all components.
   - This does make allocation/deallocation more expensive. 

## Details

Entities are IDs that reference the components that they own.
This is to make lookup cheap & easy, at the cost of memory consumption.

> Optimization note. The ID of an entity could encode an archetype, reducing the storage and accelerating arbitray lookup. This would mean that archetypes couldn't be changed however.