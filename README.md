# space

[![Crates.io][ci]][cl] ![MIT/Apache][li] [![docs.rs][di]][dl] ![LoC][lo]

[ci]: https://img.shields.io/crates/v/space.svg
[cl]: https://crates.io/crates/space/

[li]: https://img.shields.io/crates/l/specs.svg?maxAge=2592000

[di]: https://docs.rs/space/badge.svg
[dl]: https://docs.rs/space/

[lo]: https://tokei.rs/b1/github/vadixidav/space?category=code

A library that attempts to create good abstractions and implementations for common spatial data structures.

## What it currently has

- Morton encoding (z-order encoding) of 3d coordinates into and from `u64` and `u128`
- Octrees
  - Iteration
  - Gathering data from leaf nodes for internal nodes
    - Uses linear hashed octree LRU cache to speed up gathering.
    - Random sampling approach to gathering supported (e.g., run a barnes hut simulation, but limit a box's samples)
  - Performing a tree fold from the leaves to the root of the tree
  - Pointer based octrees
  - Linear hashed octrees

## What it should have

- Querying what is in a region (for colision detection)
  - This can be implemented in an abstract way using the `explore` parameter to gather operations, but convenience wrappers need to be created to search over regions (possibly using combinator functions).
- k-d trees
- R trees
- R* trees
- M trees

## What it shouldn't have

- Specific file format loading
- Physics (though it should provide enough abstractions to implement physics outside the library)

## When will `space` be `1.0`

`space` will have a stable API once [const generics](https://github.com/rust-lang/rust/issues/44580) land in Rust stable and each one of the trees supports concrete constant and dynamic dimensional data. `nalgebra` currently supports this kind of functionality via [type-level integers](https://docs.rs/nalgebra/0.16.13/nalgebra/base/dimension/trait.DimName.html). Since const generics are not far off being stabilized, `space` will simply wait for them to merge before being stabilized. In the meantime, the primary focus is on 3d trees specifically, but this is not the final destination of the API.

## Contributing

- I would love contributions to get this library to where it needs to be to support the point cloud, game,
    and physics developers in the community.
- We need more benchmarks currently, and various levels of n-body simulation would probably be a good benchmark.
- Also, see the above section `What should it have`.
