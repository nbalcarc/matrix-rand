use criterion::{black_box, criterion_group, criterion_main, Criterion};
use common::regular::multiply;


fn benchmarks(c: &mut Criterion) {
    let a = [
        3.0, 2.0, 1.0, 5.0,
        9.0, 1.0, 3.0, 0.0,
    ];

    let b = [
        2.0, 9.0, 0.0,
        1.0, 3.0, 5.0,
        2.0, 4.0, 7.0,
        8.0, 1.0, 5.0,
    ];

    let sizes = (2, 4, 3);

    c.bench_function("fib 20", |x| x.iter(|| multiply(&a, &b, sizes)));
}

criterion_group!(benches, benchmarks);
criterion_main!(benches);
