use tfhe_rs_optimization::parallel_keyswitch;

#[test]
fn test_parallel_keyswitch() {
    let keys = vec![1, 2, 3, 4];
    let mut new_keys = vec![0; 4];
    parallel_keyswitch(&keys, &mut new_keys);
    assert_eq!(new_keys, vec![2, 4, 6, 8]);
}
