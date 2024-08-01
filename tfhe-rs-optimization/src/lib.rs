pub mod integer_ops;
pub mod polynomial;
pub mod keyswitch;

// Optionally, re-export functions for easy access
pub use integer_ops::{simd_add, karatsuba};
pub use polynomial::precompute_polynomials;
pub use keyswitch::parallel_keyswitch;
