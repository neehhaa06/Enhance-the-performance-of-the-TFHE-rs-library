use tfhe_rs_optimization::precompute_polynomials;

#[test]
fn test_precompute_polynomials() {
    let polynomials = precompute_polynomials(4);
    assert_eq!(polynomials, vec![0, 1, 4, 9]);
}
