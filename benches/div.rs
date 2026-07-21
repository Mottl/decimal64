use criterion::{Criterion, black_box, criterion_group, criterion_main};
use decimal64::{DecimalU64, U8};
use rust_decimal::Decimal;
use std::str::FromStr;

fn f64_benchmark(c: &mut Criterion) {
    let one = f64::from_str("0.2").unwrap();
    let two = f64::from_str("50000").unwrap();
    let mut group = c.benchmark_group("f64");
    group.bench_function("div", |b| {
        let one = black_box(one);
        let two = black_box(two);
        b.iter(|| {
            black_box(two / one);
        })
    });
}

fn decimal64_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("decimal64");
    let one = DecimalU64::<U8>::from_str("0.2").unwrap();
    let two = DecimalU64::<U8>::from_str("50000").unwrap();
    group.bench_function("checked_div", |b| {
        let one = black_box(one);
        let two = black_box(two);
        b.iter(|| {
            black_box(two.checked_div(one).unwrap());
        })
    });
    group.bench_function("div", |b| {
        let one = black_box(one);
        let two = black_box(two);
        b.iter(|| {
            black_box(two / one);
        })
    });
}

fn rust_decimal_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("rust_decimal");
    let one = Decimal::from_str("0.2").unwrap();
    let two = Decimal::from_str("50000").unwrap();
    group.bench_function("checked_div", |b| {
        let one = black_box(one);
        let two = black_box(two);
        b.iter(|| {
            black_box(two.checked_div(one).unwrap());
        })
    });
    group.bench_function("div", |b| {
        let one = black_box(one);
        let two = black_box(two);
        b.iter(|| {
            black_box(two / one);
        })
    });
}

criterion_group!(benches, f64_benchmark, decimal64_benchmark, rust_decimal_benchmark);
criterion_main!(benches);
