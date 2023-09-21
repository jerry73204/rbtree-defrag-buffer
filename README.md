# Defragmentation Buffer backed by Red-Back Tree in Rust

The implemented defragmentation buffer maintains inserted and free
intervals. It supports fast interval insertion down to hundreds of
nanoseconds.

## Example

The example code creates a defrag buffer for a 10-byte interval. It
inserts the range 2..7, and the buffers gives two free intervals, one
is 0..2 and the other is 7..10.

```rust
use rbtree_defrag_buffer::DefragBuf;

let mut buf = DefragBuf::new(10);
buf.insert(2..7).unwrap();

let free_invs: Vec<_> = buf.free_intervals().collect();
assert_eq!(free_invs, vec![0..2, 7..10]);
```


## License

This software is distributed under MIT license. Please check the
[LICENSE.txt](LICENSE.txt) file to see the full license.
