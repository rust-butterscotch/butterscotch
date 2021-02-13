/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

use crate::{ChunkSize, ChunkyVec};

#[test]
fn test() -> Result<(), String> {
    let mut v = ChunkyVec::<usize>::new(ChunkSize::Elements(1024));
    do_test(&mut v, 8)
}

fn do_test(v: &mut ChunkyVec::<usize>, limit: usize) -> Result<(), String> {
    let chunk_size = v.chunk_size();

    // // Test Initial State // //
    assert_eq!(v.len(),         0);
    assert_eq!(v.chunks_used(), 0);
    assert_eq!(v.capacity(),    0);
    assert!(v.is_empty());
    v.check_integrity()?;

    do_recuse_test(v, 1, limit)?;

    // // Ensure Exit State // //
    assert_eq!(v.len(),                     0);
    assert_eq!(v.capacity(), limit*chunk_size);
    assert_eq!(v.chunks_used(),             0);

    // // Test Shrink to Fit // //
    v.push(0);
    v.shrink_to_fit();
    assert_eq!(v.chunks_used(),       1);
    assert_eq!(v.len(),               1);
    assert_eq!(v.capacity(), chunk_size);

    Ok(())
}

fn do_recuse_test(v: &mut ChunkyVec::<usize>, mut blocks: usize, limit: usize) -> Result<(), String>  {
    let chunk_size = v.chunk_size();

    // // Push Data // //
    push_values(v,              blocks*chunk_size)?;
    assert_eq!(v.capacity(),    blocks*chunk_size);
    assert_eq!(v.chunks_used(), blocks           );

    // // Pop Data // //
    pop_values(v,            blocks*chunk_size)?;
    assert_eq!(v.capacity(), blocks*chunk_size);
    assert_eq!(v.chunks_used(), 0);

    // // Push Extra Block // //
    push_values(v,              blocks*chunk_size)?;
    assert_eq!(v.capacity(),    blocks*chunk_size);
    assert_eq!(v.chunks_used(), blocks);
    push_values(v, 1)?;
    blocks += 1;
    assert_eq!(v.capacity(),    blocks*chunk_size);
    assert_eq!(v.chunks_used(), blocks);

    // // Clear Data // //
    v.clear();
    assert_eq!(v.len(),                      0);
    assert_eq!(v.capacity(), blocks*chunk_size);
    assert_eq!(v.chunks_used(),              0);
    assert!(v.is_empty());
    v.check_integrity()?;

    // // Recurse // //
    if blocks >= limit {
        Ok(())
    } else {
        do_recuse_test(v, blocks, limit)
    }
}


pub fn push_values(v: &mut ChunkyVec<usize>, count: usize) -> Result<(), String> {
    let start_len = v.len();

    // // Add Data // //
    for i in 0..count {
        v.push(i);
        assert!(!v.is_empty());
        assert_eq!(v.len(), start_len + i+1);
        v.check_integrity()?;
    }

    Ok(())
}

pub fn pop_values(v: &mut ChunkyVec<usize>, count: usize) -> Result<(), String> {
    let start_len = v.len();

    // // Add Data // //
    for i in 1..=count {
        v.pop();
        assert_eq!(v.len()+i, start_len);
        v.check_integrity()?;
    }

    if count == start_len {
        assert!(v.is_empty());
    }

    Ok(())
}
