# Butterscotch Game Engine

Welcome to the Butterscotch modular game engine.

The engine is still early in development by seeks to provide the following features:
 - Support for Linux, WebAssembly and Windows - in that order.
 - Fast 2D WebGPU rendering pipeline.
   - Hopefully vector-based, that'd be cool.
 - ECS-based architecture designed around functional reactive systems
   - This is not an entire FRP-implementation, and it doesn't require pure-functions/monads/whatever.
   - Reactive systems define what type of mutation or event they wish to react to.
     - Mutations include create entitiy, destroy entitiy, attach component, detach component, mutate component
     - Events are user-defined. Containing information on triggering entities, as well as custom data.
   - These systems, when triggered, output a combination of mutations and events.
   - Systems which cross the boundry (ie. Networking, Input, Rendering) should not exist within the ECS
     - Input systems (ie. Filesystem, Networking, Input) should communicate with the ECS via Events.
     - Output systems (ie. Rendering, Audio) should immutably observe from the outside.
     - Most boundry systems are a really a combination of input and output, but should be minimized where possible.
   - Systems are assumed to act on sets of entities in parallel, even if single-threaded.
     - This means that effects from each invocation of the system are not visible to other invocations of the system until they are all finalized.
     - At the same time, systems are also assumed to be aware of side-effects from other invocations and as such will not self-trigger.
   - This is still an idea in early stages, with lots of questions:
     - How do we create a new entity?
     - How do we destroy an entity?
     - How do we attach new components?
     - How do we track mutations?
     - How do we not make this an absolute pain to work with?
     - How do we optimize it for the general case?
     - How do we handle reacting to single, specific, entities?
     - How do reactive systems communicate with boundy systems? Event streams probbably?
     - How do we handle reaction loops?
