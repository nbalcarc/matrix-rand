use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion};
use rand;

use common::regular;


fn benchmarks(c: &mut Criterion) {
    let sizes = (2_048, 1_536, 1_024);

    let a: Vec<f32> = (0..sizes.0*sizes.1).into_iter().map(|_| rand::random()).collect();
    let b: Vec<f32> = (0..sizes.1*sizes.2).into_iter().map(|_| rand::random()).collect();
    c.bench_function("regular float", |x| x.iter(|| regular::multiply_float(&a, &b, sizes)));

    let a: Vec<i32> = (0..sizes.0*sizes.1).into_iter().map(|_| rand::random()).collect();
    let b: Vec<i32> = (0..sizes.1*sizes.2).into_iter().map(|_| rand::random()).collect();
    c.bench_function("regular int", |x| x.iter(|| regular::multiply_int(&a, &b, sizes)));
}

criterion_group!{
    name = benches;
    config = Criterion::default().sample_size(10).measurement_time(Duration::from_secs(300));
    targets = benchmarks
}
criterion_main!(benches);


