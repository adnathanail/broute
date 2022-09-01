## Learning rust
### Rustlings
https://cheats.rs/
https://betterprogramming.pub/how-to-structure-unit-tests-in-rust-cc4945536a32

### Understanding mut and & https://stackoverflow.com/a/67674348/9261263
Modules and folders https://learning-rust.github.io/docs/d3.modules.html
Idioms https://rust-unofficial.github.io/patterns/idioms/index.html

## Routing algorithms 

Dijkstra implementation https://brilliant.org/wiki/dijkstras-short-path-finder/

### Optimisation ideas
https://en.wikipedia.org/wiki/Contraction_hierarchies
https://github.com/graphhopper/graphhopper/blob/master/docs/core/low-level-api.md

### Potential sources of inefficiency
Graph implementation
Concurrency
Storage medium (db? File? Binary file? Memory?)

## Why rust is excellent

Strict typing like Haskell
Deals with memory completely safely, and efficiently

Box
- Box allows you to easily allocate memory on the heap, and then it is automatically deallocated when the usage goes out of scope!
- Also, you can access the value in the Box exactly as you would without Box there!!

collect
- The exact same code can give a different output, just by giving it a different type (see rustlings iterators3)

## Challenges

### Ordering f32

https://www.reddit.com/r/rust/comments/29kia3/no_ord_for_f32/
https://github.com/rust-lang/rfcs/issues/1249
https://stackoverflow.com/questions/26489701/why-does-rust-not-implement-total-ordering-via-the-ord-trait-for-f64-and-f32

IEEE 754 floats don't define a total order

I want to represent my distances as floats

