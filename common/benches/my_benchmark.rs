use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion};
use rand;

use common::{monte, regular};


fn benchmarks(c: &mut Criterion) {
    let sizes = (2_048, 1_536, 1_024);

    let a_float: Vec<f32> = (0..sizes.0*sizes.1).into_iter().map(|_| rand::random()).collect();
    let b_float: Vec<f32> = (0..sizes.1*sizes.2).into_iter().map(|_| rand::random()).collect();
    let a_int: Vec<i32> = (0..sizes.0*sizes.1).into_iter().map(|_| rand::random()).collect();
    let b_int: Vec<i32> = (0..sizes.1*sizes.2).into_iter().map(|_| rand::random()).collect();
    c.bench_function("regular float", |x| x.iter(|| regular::multiply_float(&a_float, &b_float, sizes)));
    c.bench_function("regular int", |x| x.iter(|| regular::multiply_int(&a_int, &b_int, sizes)));
    c.bench_function("monte float 1152", |x| x.iter(|| monte::multiply_float(&a_float, &b_float, sizes, 1152)));
    c.bench_function("monte int 1152", |x| x.iter(|| monte::multiply_int(&a_int, &b_int, sizes, 1152)));
    c.bench_function("monte float 768", |x| x.iter(|| monte::multiply_float(&a_float, &b_float, sizes, 768)));
    c.bench_function("monte int 768", |x| x.iter(|| monte::multiply_int(&a_int, &b_int, sizes, 768)));
    c.bench_function("monte float 384", |x| x.iter(|| monte::multiply_float(&a_float, &b_float, sizes, 384)));
    c.bench_function("monte int 384", |x| x.iter(|| monte::multiply_int(&a_int, &b_int, sizes, 384)));
    c.bench_function("monte float 192", |x| x.iter(|| monte::multiply_float(&a_float, &b_float, sizes, 192)));
    c.bench_function("monte int 192", |x| x.iter(|| monte::multiply_int(&a_int, &b_int, sizes, 192)));
}

criterion_group!{
    name = benches;
    config = Criterion::default().sample_size(10).measurement_time(Duration::from_secs(300));
    targets = benchmarks
}
criterion_main!(benches);


