use std::arch::x86_64::*; // Import SIMD intrinsics

/// Adds two arrays of 64-bit integers 
pub fn simd_add(a: &[u64], b: &[u64], result: &mut [u64]) {
    assert_eq!(a.len(), b.len());
    assert_eq!(a.len(), result.len());

    unsafe {
        let len = a.len();
        let mut i = 0;
        
        while i + 4 <= len {
            let va = _mm256_loadu_si256(a[i..].as_ptr() as *const __m256i);
            let vb = _mm256_loadu_si256(b[i..].as_ptr() as *const __m256i);
            let vr = _mm256_add_epi64(va, vb);
            _mm256_storeu_si256(result[i..].as_mut_ptr() as *mut __m256i, vr);
            i += 4;
        }

        while i < len {
            result[i] = a[i] + b[i];
            i += 1;
        }
    }
}

/// Multiplies two large integers using the Karatsuba algorithm
pub fn karatsuba(x: &[u64], y: &[u64]) -> Vec<u64> {
    let n = x.len();
    if n == 1 {
        return vec![x[0] * y[0]];
    }
    let mid = n / 2;
    let (x_low, x_high) = x.split_at(mid);
    let (y_low, y_high) = y.split_at(mid);

    let z0 = karatsuba(x_low, y_low);
    let z2 = karatsuba(x_high, y_high);

    let x_low_high: Vec<u64> = x_low.iter().chain(x_high).copied().collect();
    let y_low_high: Vec<u64> = y_low.iter().chain(y_high).copied().collect();
    let z1 = karatsuba(&x_low_high, &y_low_high);

    let mut result = vec![0; 2 * n - 1];
    for (i, val) in z0.iter().enumerate() {
        result[i] += val;
    }
    for (i, val) in z1.iter().enumerate() {
        result[mid + i] += val;
    }
    for (i, val) in z2.iter().enumerate() {
        result[2 * mid + i] += val;
    }
    result
}
