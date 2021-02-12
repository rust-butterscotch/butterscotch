# ChunkyVec

This crate provides an implementation of a vector-like structure that occupies
a non-contiguous section of memory. This is similar to C++'s typical std::deque
implementation, with a few caveats. C++'s deque doesn't provide as much benifit
to rust, as its address/iterator stability flies in the face of rust's approach
to aliasing.

However, it does provide good performance charateristics for very large collections 
of objects that are added individually. This is because growing the vector is very
cheap, as large chunks can be added to the end at worst for the cost of allocating
such blocks and copying a comparitively lower number of small structs.

Keep in mind that ChunkyVec has similar performance characteristics for most 
operations, it is very much NOT a double-ended queue like std::dequeue. It is 
designed for inserting/removing from the end of the queue, inserting or removing 
from anywhere else will have even worse performance than vectors, as the data 
cannot all be moved at once. If you need to remove from the middle of the vector,
please use swap_remove - it doesn't maintain order but should be nearly as fast
as a vector.

## Details

ChunkyVec is implemented entirely using standard library vectors and safe code.
This should mean that it's entirely safe, but also means that the memory it 
requires and potentially the performance is not as high as it could be with a
specialized chunk data structure.

## TODO

### Not Implemented Currently
- append
- extend_from_slice
- remove_item
- retain
- dedup
- dedup_by
- dedup_by_key
- leak
- into_boxed_slice
- into_raw_parts
- from_raw_parts
- split_off
- splice
- drain
- drain_filter
- set_len

### Probbably Can't Implement Properly
- spare_capacity_mut
- as_mut_ptr
- as_mut_slice
- as_ptr
- as_slice
- try_reserve
- try_reserve_exact

### Misc
 - Improve testing
 - Write documentation
