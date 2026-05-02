use criterion::{
    BenchmarkGroup, BenchmarkId, Criterion, criterion_group, criterion_main, measurement::WallTime,
};
use std::env;
use std::hint::black_box;
use std::str::FromStr;

// primitive f64
fn bench_f64(group: &mut BenchmarkGroup<'_, WallTime>, sample: usize) {
    for iexp in (0..=36).step_by(sample) {
        let man = 10_i128.pow(iexp);

        let a = man as f64;

        group.bench_with_input(BenchmarkId::new("f64", iexp), &(a, a), |b, i| {
            b.iter(|| black_box(i.0 * i.1))
        });
    }
}

// crate: rust_decimal
fn bench_rust_decimal(group: &mut BenchmarkGroup<'_, WallTime>, sample: usize) {
    use rust_decimal::prelude::Decimal;

    for iexp in (0..=28).step_by(sample) {
        let man = 10_i128.pow(iexp);

        let a = Decimal::from_i128_with_scale(man, iexp);

        group.bench_with_input(BenchmarkId::new("rust_decimal", iexp), &(a, a), |b, i| {
            b.iter(|| black_box(i.0 * i.1))
        });
    }
}

// crate: bigdecimal
fn bench_bigdecimal(group: &mut BenchmarkGroup<'_, WallTime>, sample: usize) {
    use bigdecimal::{BigDecimal, num_bigint::BigInt};

    for iexp in (0..=38).step_by(sample) {
        let man = BigInt::from(10_i128.pow(iexp));

        let a = BigDecimal::from_bigint(man.clone(), iexp as i64);
        let b = a.clone();

        group.bench_with_input(BenchmarkId::new("bigdecimal", iexp), &(a, b), |b, i| {
            b.iter(|| black_box(&i.0 * &i.1))
        });
    }
}

// crate: decimax
fn bench_decimax(group: &mut BenchmarkGroup<'_, WallTime>, sample: usize) {
    use decimax::{Dec64, Dec128};

    for iexp in (0..=36).step_by(sample) {
        let man = 10_i128.pow(iexp);

        let a = Dec128::from_parts(man, iexp.min(31));

        group.bench_with_input(BenchmarkId::new("decimax:128", iexp), &(a, a), |b, i| {
            b.iter(|| black_box(i.0 * i.1))
        });
    }

    for iexp in (0..=17).step_by(sample) {
        let man = 10_i64.pow(iexp);

        let a = Dec64::from_parts(man, iexp.min(15));

        group.bench_with_input(BenchmarkId::new("decimax:64", iexp), &(a, a), |b, i| {
            b.iter(|| black_box(i.0 * i.1))
        });
    }
}

// crate: fastnum
fn bench_fastnum(group: &mut BenchmarkGroup<'_, WallTime>, sample: usize) {
    use fastnum::{
        D64, D128,
        bint::UInt,
        decimal::{Context, Sign},
    };

    for iexp in (0..=38).step_by(sample) {
        let man = UInt::<2>::from_u128(10_u128.pow(iexp)).unwrap();

        let a = D128::from_parts(man, iexp as i32, Sign::Plus, Context::default());
        let b = a.clone();

        group.bench_with_input(BenchmarkId::new("fastnum:128", iexp), &(a, b), |b, i| {
            b.iter(|| black_box(i.0 * i.1))
        });
    }

    for iexp in (0..=19).step_by(sample) {
        let man = UInt::<1>::from_u64(10_u64.pow(iexp));

        let a = D64::from_parts(man.clone(), iexp as i32, Sign::Plus, Context::default());
        let b = a.clone();

        group.bench_with_input(BenchmarkId::new("fastnum:64", iexp), &(a, b), |b, i| {
            b.iter(|| black_box(i.0 * i.1))
        });
    }
}

// crate: primitive_fixed_point_decimal
fn bench_primitive_fixed_point_decimal(group: &mut BenchmarkGroup<'_, WallTime>, sample: usize) {
    use primitive_fixed_point_decimal::OobScaleFpdec;
    type Dec128 = OobScaleFpdec<i128>;
    type Dec64 = OobScaleFpdec<i64>;

    for iexp in (0..=38).step_by(sample) {
        let man = 10_i128.pow(iexp);

        let a = Dec128::from_mantissa(man);

        let diff_scale = (iexp as i32 * 2 - 38).max(0);

        group.bench_with_input(BenchmarkId::new("prim-fpdec:128", iexp), &(a, a), |b, i| {
            b.iter(|| black_box(i.0.checked_mul(i.1, diff_scale)))
        });
    }

    for iexp in (0..=19).step_by(sample) {
        let man = 10_i64.pow(iexp);

        let a = Dec64::from_mantissa(man);

        let diff_scale = (iexp as i32 * 2 - 19).max(0);

        group.bench_with_input(BenchmarkId::new("prim-fpdec:64", iexp), &(a, a), |b, i| {
            b.iter(|| black_box(i.0.checked_mul(i.1, diff_scale)))
        });
    }
}

// entry
fn criterion_benchmark(c: &mut Criterion) {
    let machine = env::var("MACHINE").unwrap_or_default();
    let sample = env::var("SAMPLE")
        .map(|s| usize::from_str(&s).expect("invalid SAMPLE"))
        .unwrap_or(1);

    let mut group = c.benchmark_group(format!("mulplication{machine}"));
    // bench_bigdecimal(&mut group, sample);
    // bench_fastnum(&mut group, sample);
    bench_rust_decimal(&mut group, sample);
    bench_decimax(&mut group, sample);
    bench_primitive_fixed_point_decimal(&mut group, sample);
    bench_f64(&mut group, sample);
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
