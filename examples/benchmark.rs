use clap::Parser;
use rand::seq::SliceRandom;
use rbtree_defrag_buffer::DefragBuf;
use std::time::Instant;

/// Measure total and average insertion time to the defragmentation
/// buffer.
#[derive(Debug, Parser)]
struct Opts {
    /// Maximum size for the defragmentation buffer.
    #[clap(long, default_value = "1048576")]
    pub size: usize,
}

fn main() {
    let Opts { size } = Opts::parse();

    let mut buf = DefragBuf::new(size);
    let mut rng = rand::thread_rng();

    let mut indices: Vec<usize> = (0..size).collect();
    indices.shuffle(&mut rng);

    let since = Instant::now();

    for idx in indices {
        buf.insert(idx..(idx + 1)).unwrap();
    }

    let elapsed = since.elapsed();
    let avg = elapsed / size as u32;

    println!("size\t{size}B");
    println!("total\t{elapsed:?}");
    println!("avg\t{avg:?}");
}
