/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

use crate::{ChunkSize, ChunkyVec};

#[test]
fn test() -> Result<(), String> {
    let mut v = ChunkyVec::<usize>::new(ChunkSize::Elements(1024));
    do_test::<1024>(&mut v, 8)
}

fn do_test<const N: usize>(v: &mut ChunkyVec::<usize>, limit: usize) -> Result<(), String> {
    // // Test Initial State // //
    assert_eq!(v.len(),          0);
    assert_eq!(v.chunk_length(), 0);
    assert_eq!(v.capacity(),     0);
    assert!(v.is_empty());
    v.check_integrity()?;

    do_recuse_test::<N>(v, 1, limit)?;

    // // Ensure Exit State // //
    assert_eq!(v.len(),      0);
    assert_eq!(v.capacity(), limit*N);
    assert_eq!(v.chunk_length(), 0);

    // // Test Shrink to Fit // //
    v.push(0);
    v.shrink_to_fit();
    assert_eq!(v.chunk_length(), 1);
    assert_eq!(v.len(),      1);
    assert_eq!(v.capacity(), N);

    Ok(())
}

fn do_recuse_test<const N: usize>(v: &mut ChunkyVec::<usize>, mut blocks: usize, limit: usize) -> Result<(), String>  {

    // // Push Data // //
    push_values::<N>(v,           blocks*N)?;
    assert_eq!(v.capacity(), blocks*N);
    assert_eq!(v.chunk_length(), blocks);

    // // Pop Data // //
    pop_values::<N>(v,            blocks*N)?;
    assert_eq!(v.capacity(), blocks*N);
    assert_eq!(v.chunk_length(), 0);

    // // Push Extra Block // //
    push_values::<N>(v,     blocks*N)?;
    assert_eq!(v.capacity(),blocks*N);
    assert_eq!(v.chunk_length(), blocks);
    push_values::<N>(v, 1)?;
    blocks += 1;
    assert_eq!(v.capacity(), blocks*N);
    assert_eq!(v.chunk_length(), blocks);

    // // Clear Data // //
    v.clear();
    assert_eq!(v.len(),      0);
    assert_eq!(v.capacity(), blocks*N);
    assert_eq!(v.chunk_length(), 0);
    assert!(v.is_empty());
    v.check_integrity()?;

    // // Recurse // //
    if blocks >= limit {
        Ok(())
    } else {
        do_recuse_test::<N>(v, blocks, limit)
    }
}


pub fn push_values<const N: usize>(v: &mut ChunkyVec<usize>, count: usize) -> Result<(), String> {
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

pub fn pop_values<const N: usize>(v: &mut ChunkyVec<usize>, count: usize) -> Result<(), String> {
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
