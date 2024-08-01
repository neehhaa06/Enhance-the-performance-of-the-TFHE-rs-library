use criterion::{criterion_group, criterion_main, Criterion};
use tfhe_rs_optimization::{simd_add, karatsuba, precompute_polynomials, parallel_keyswitch};

fn benchmark_simd_add(c: &mut Criterion) {
    let a = vec![1u64; 1_000_000];
    let b = vec![2u64; 1_000_000];
    let mut result = vec![0u64; 1_000_000];
    
    c.bench_function("simd_add", |b| {
        b.iter(|| simd_add(&a, &b, &mut result))
    });
}

fn benchmark_karatsuba(c: &mut Criterion) {
    let x = vec![1u64; 1024];
    let y = vec![2u64; 1024];
    
    c.bench_function("karatsuba", |b| {
        b.iter(|| karatsuba(&x, &y))
    });
}

fn benchmark_precompute_polynomials(c: &mut Criterion) {
    c.bench_function("precompute_polynomials", |b| {
        b.iter(|| precompute_polynomials(1024))
    });
}

fn benchmark_parallel_keyswitch(c: &mut Criterion) {
    let keys = vec![1u64; 1_000_000];
    let mut new_keys = vec![0u64; 1_000_000];
    
    c.bench_function("parallel_keyswitch", |b| {
        b.iter(|| parallel_keyswitch(&keys, &mut new_keys))
    });
}

criterion_group!(benches, benchmark_simd_add, benchmark_karatsuba, benchmark_precompute_polynomials, benchmark_parallel_keyswitch);
criterion_main!(benches);
