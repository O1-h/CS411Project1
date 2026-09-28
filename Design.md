# Project 1: Design

## What's shared read-only and what's mutated per-thread

- Shared read-only: 
Scene is shared in Arc<Scene> with every worker through thread::scope

- Mutated per-thread: 
every thread has its own Vec<Pixel> that is not shared until it is done and it is joined 
with all the others in order

## Tiling strategy

I chose row band tiles over rectangular blocks because 
it was easier to join and split since they are split only one way.


## Note about ahmdals
the ahmdals predicted speed up is different than the  speedup column
due to the render speedup only measuring the render time and not any of the sequential portions
like the scene setup nad loading. 