/// programmable bootstrapping
pub fn precompute_polynomials(degree: usize) -> Vec<u64> {
    (0..degree).map(|i| (i as u64).pow(2)).collect()
}
