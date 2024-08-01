use tfhe_rs_optimization::{simd_add, karatsuba};

#[test]
fn test_simd_add() {
    let a = vec![1, 2, 3, 4];
    let b = vec![5, 6, 7, 8];
    let mut result = vec![0; 4];
    simd_add(&a, &b, &mut result);
    assert_eq!(result, vec![6, 8, 10, 12]);
}

#[test]
fn test_karatsuba() {
    let x = vec![1, 2];
    let y = vec![3, 4];
    let result = karatsuba(&x, &y);
    assert_eq!(result, vec![3, 10, 8]);
}
