use rayon::prelude::*; // Import Rayon for parallel processing

/// Parallelizes keyswitching operations
pub fn parallel_keyswitch(keys: &[u64], new_keys: &mut [u64]) {
    new_keys.par_iter_mut().enumerate().for_each(|(i, new_key)| {
        *new_key = keys[i] * 2; // Example keyswitching operation
    });
}
