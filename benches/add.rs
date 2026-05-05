#![feature(f128)]

use criterion::{
    BenchmarkGroup, BenchmarkId, Criterion, criterion_group, criterion_main, measurement::WallTime,
};
use std::env;
use std::hint::black_box;
use std::str::FromStr;

// primitive f64
fn bench_f64(group: &mut BenchmarkGroup<'_, WallTime>, sample: usize, rescale: bool) {
    for iexp in (0..=36).step_by(sample) {
        let man = 10_i128.pow(iexp);

        let a = man as f64;
        let b = if rescale { a.powi(10) } else { a };

        group.bench_with_input(BenchmarkId::new("f64", iexp), &(a, b), |b, i| {
            b.iter(|| black_box(i.0 + i.1))
        });
    }
}

// primitive f128
fn bench_f128(group: &mut BenchmarkGroup<'_, WallTime>, sample: usize, rescale: bool) {
    for iexp in (0..=36).step_by(sample) {
        let man = 10_i128.pow(iexp);

        let a = man as f128;
        let b = if rescale { a.powi(10) } else { a };

        group.bench_with_input(BenchmarkId::new("f128", iexp), &(a, b), |b, i| {
            b.iter(|| black_box(i.0 + i.1))
        });
    }
}

// crate: rust_decimal
fn bench_rust_decimal(group: &mut BenchmarkGroup<'_, WallTime>, sample: usize, rescale: bool) {
    use rust_decimal::prelude::Decimal;

    let b_scale = if rescale { 10 } else { 0 };

    for iexp in (0..=28).step_by(sample) {
        let man = 10_i128.pow(iexp);

        let a = Decimal::from_i128_with_scale(man, 0);
        let b = Decimal::from_i128_with_scale(man, b_scale);

        group.bench_with_input(BenchmarkId::new("rust_decimal", iexp), &(a, b), |b, i| {
            b.iter(|| black_box(i.0 + i.1))
        });
    }
}

// crate: bigdecimal
fn bench_bigdecimal(group: &mut BenchmarkGroup<'_, WallTime>, sample: usize, rescale: bool) {
    use bigdecimal::{BigDecimal, num_bigint::BigInt};

    let b_scale = if rescale { 10 } else { 0 };

    for iexp in (0..=38).step_by(sample) {
        let man = BigInt::from(10_i128.pow(iexp));

        let a = BigDecimal::from_bigint(man.clone(), 0);
        let b = BigDecimal::from_bigint(man, b_scale);

        group.bench_with_input(BenchmarkId::new("bigdecimal", iexp), &(a, b), |b, i| {
            b.iter(|| black_box(&i.0 + &i.1))
        });
    }
}

// crate: decimax
fn bench_decimax(group: &mut BenchmarkGroup<'_, WallTime>, sample: usize, rescale: bool) {
    use decimax::{Dec64, Dec128};

    let b_scale = if rescale { 10 } else { 0 };

    for iexp in (0..=36).step_by(sample) {
        let man = 10_i128.pow(iexp);

        let a = Dec128::from_parts(man, 0);
        let b = Dec128::from_parts(man, b_scale);

        group.bench_with_input(BenchmarkId::new("decimax:128", iexp), &(a, b), |b, i| {
            b.iter(|| black_box(i.0 + i.1))
        });
    }

    for iexp in (0..=17).step_by(sample) {
        let man = 10_i64.pow(iexp);

        let a = Dec64::from_parts(man, 0);
        let b = Dec64::from_parts(man, b_scale);

        group.bench_with_input(BenchmarkId::new("decimax:64", iexp), &(a, b), |b, i| {
            b.iter(|| black_box(i.0 + i.1))
        });
    }
}

// crate: fastnum
fn bench_fastnum(group: &mut BenchmarkGroup<'_, WallTime>, sample: usize, rescale: bool) {
    use fastnum::{
        D64, D128,
        bint::UInt,
        decimal::{Context, Sign},
    };

    let b_scale = if rescale { 10 } else { 0 };

    for iexp in (0..=38).step_by(sample) {
        let man = UInt::<2>::from_u128(10_u128.pow(iexp)).unwrap();

        let a = D128::from_parts(man.clone(), 0, Sign::Plus, Context::default());
        let b = D128::from_parts(man, b_scale, Sign::Plus, Context::default());

        group.bench_with_input(BenchmarkId::new("fastnum:128", iexp), &(a, b), |b, i| {
            b.iter(|| black_box(i.0 + i.1))
        });
    }

    for iexp in (0..=19).step_by(sample) {
        let man = UInt::<1>::from_u64(10_u64.pow(iexp));

        let a = D64::from_parts(man.clone(), 0, Sign::Plus, Context::default());
        let b = D64::from_parts(man, b_scale, Sign::Plus, Context::default());

        group.bench_with_input(BenchmarkId::new("fastnum:64", iexp), &(a, b), |b, i| {
            b.iter(|| black_box(i.0 + i.1))
        });
    }
}

// crate: primitive_fixed_point_decimal
fn bench_primitive_fixed_point_decimal(group: &mut BenchmarkGroup<'_, WallTime>, sample: usize) {
    use primitive_fixed_point_decimal::ConstScaleFpdec;
    type Dec128 = ConstScaleFpdec<i128, 10>;
    type Dec64 = ConstScaleFpdec<i64, 5>;

    for iexp in (0..=38).step_by(sample) {
        let man = 10_i128.pow(iexp);

        let a = Dec128::from_mantissa(man);

        group.bench_with_input(BenchmarkId::new("prim-fpdec:128", iexp), &(a, a), |b, i| {
            b.iter(|| black_box(i.0 + i.1))
        });
    }

    for iexp in (0..=19).step_by(sample) {
        let man = 10_i64.pow(iexp);

        let a = Dec64::from_mantissa(man);

        group.bench_with_input(BenchmarkId::new("prim-fpdec:64", iexp), &(a, a), |b, i| {
            b.iter(|| black_box(i.0 + i.1))
        });
    }
}

// entry
fn criterion_benchmark(c: &mut Criterion) {
    let machine = env::var("MACHINE").unwrap_or_default();
    let sample = env::var("SAMPLE")
        .map(|s| usize::from_str(&s).expect("invalid SAMPLE"))
        .unwrap_or(1);

    let mut group = c.benchmark_group(format!("addition:pure{machine}"));
    // bench_bigdecimal(&mut group, sample, false);
    bench_fastnum(&mut group, sample, false);
    bench_rust_decimal(&mut group, sample, false);
    bench_decimax(&mut group, sample, false);
    bench_primitive_fixed_point_decimal(&mut group, sample);
    bench_f128(&mut group, sample, false);
    bench_f64(&mut group, sample, false);
    group.finish();

    let mut group = c.benchmark_group(format!("addition:rescale{machine}"));
    bench_bigdecimal(&mut group, sample, true);
    bench_fastnum(&mut group, sample, true);
    bench_rust_decimal(&mut group, sample, true);
    bench_decimax(&mut group, sample, true);
    bench_f128(&mut group, sample, true);
    bench_f64(&mut group, sample, true);
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
