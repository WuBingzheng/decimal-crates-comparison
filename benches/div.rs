use criterion::{
    BenchmarkGroup, BenchmarkId, Criterion, criterion_group, criterion_main, measurement::WallTime,
};
use std::env;
use std::str::FromStr;

// primitive f64
fn bench_f64(group: &mut BenchmarkGroup<'_, WallTime>, sample: usize, extra: bool) {
    let d = 10_f64.powi(8) + extra as u8 as f64;

    for iexp in (0..=36).step_by(sample) {
        let man = 10_i128.pow(iexp);

        let n = man as f64;

        group.bench_with_input(BenchmarkId::new("f64", iexp), &(n, d), |b, i| {
            b.iter(|| i.0 / i.1)
        });
    }
}

// crate: rust_decimal
fn bench_rust_decimal(group: &mut BenchmarkGroup<'_, WallTime>, sample: usize, extra: bool) {
    use rust_decimal::prelude::Decimal;

    let d_man = 10_i128.pow(8) + extra as i128;
    let d = Decimal::from_i128_with_scale(d_man, 10);

    for iexp in (0..=28).step_by(sample) {
        let man = 10_i128.pow(iexp);

        let n = Decimal::from_i128_with_scale(man, 10);

        group.bench_with_input(BenchmarkId::new("rust_decimal", iexp), &(n, d), |b, i| {
            b.iter(|| i.0 / i.1)
        });
    }
}

// crate: bigdecimal
fn bench_bigdecimal(group: &mut BenchmarkGroup<'_, WallTime>, sample: usize, extra: bool) {
    use bigdecimal::{BigDecimal, num_bigint::BigInt};

    let d_man = BigInt::from(10_i128.pow(8) + extra as i128);
    let d = BigDecimal::from_bigint(d_man, 10);

    for iexp in (0..=38).step_by(sample) {
        let man = BigInt::from(10_i128.pow(iexp));

        let n = BigDecimal::from_bigint(man, 10);

        group.bench_with_input(
            BenchmarkId::new("bigdecimal", iexp),
            &(n, d.clone()),
            |b, i| b.iter(|| &i.0 / &i.1),
        );
    }
}

// crate: decimax
fn bench_decimax(group: &mut BenchmarkGroup<'_, WallTime>, sample: usize, extra: bool) {
    use decimax::{Dec64, Dec128};

    let d_man = 10_i128.pow(8) + extra as i128;
    let d = Dec128::from_parts(d_man, 10);

    for iexp in (0..=36).step_by(sample) {
        let man = 10_i128.pow(iexp);

        let n = Dec128::from_parts(man, 10);

        group.bench_with_input(BenchmarkId::new("decimax:128", iexp), &(n, d), |b, i| {
            b.iter(|| i.0 / i.1)
        });
    }

    let d_man = 10_i64.pow(8) + extra as i64;
    let d = Dec64::from_parts(d_man, 10);

    for iexp in (0..=17).step_by(sample) {
        let man = 10_i64.pow(iexp);

        let n = Dec64::from_parts(man, 10);

        group.bench_with_input(BenchmarkId::new("decimax:64", iexp), &(n, d), |b, i| {
            b.iter(|| i.0 / i.1)
        });
    }
}

// crate: fastnum
fn bench_fastnum(group: &mut BenchmarkGroup<'_, WallTime>, sample: usize, extra: bool) {
    use fastnum::{
        D64, D128,
        bint::UInt,
        decimal::{Context, Sign},
    };

    let d_man = UInt::<2>::from_u128(10_u128.pow(8) + extra as u128).unwrap();
    let d = D128::from_parts(d_man, 10, Sign::Plus, Context::default());

    for iexp in (0..=38).step_by(sample) {
        let man = UInt::<2>::from_u128(10_u128.pow(iexp)).unwrap();

        let n = D128::from_parts(man, 10, Sign::Plus, Context::default());

        group.bench_with_input(
            BenchmarkId::new("fastnum:128", iexp),
            &(n, d.clone()),
            |b, i| b.iter(|| i.0 / i.1),
        );
    }

    let d_man = UInt::<1>::from_u64(10_u64.pow(8) + extra as u64);
    let d = D64::from_parts(d_man, 10, Sign::Plus, Context::default());
    for iexp in (0..=19).step_by(sample) {
        let man = UInt::<1>::from_u64(10_u64.pow(iexp));

        let n = D64::from_parts(man, 0, Sign::Plus, Context::default());

        group.bench_with_input(
            BenchmarkId::new("fastnum:64", iexp),
            &(n, d.clone()),
            |b, i| b.iter(|| i.0 / i.1),
        );
    }
}

// crate: primitive_fixed_point_decimal
fn bench_primitive_fixed_point_decimal(
    group: &mut BenchmarkGroup<'_, WallTime>,
    sample: usize,
    extra: bool,
) {
    use primitive_fixed_point_decimal::ConstScaleFpdec;
    type Dec128 = ConstScaleFpdec<i128, 8>;
    type Dec64 = ConstScaleFpdec<i64, 8>;

    let d_man = 10_i128.pow(8);
    let d = Dec128::from_mantissa(d_man + extra as i128);

    for iexp in (0..=38).step_by(sample) {
        let man = 10_i128.pow(iexp);

        let n = Dec128::from_mantissa(man);

        group.bench_with_input(BenchmarkId::new("prim-fpdec:128", iexp), &(n, d), |b, i| {
            b.iter(|| i.0 / i.1)
        });
    }

    let d_man = 10_i64.pow(8);
    let d = Dec64::from_mantissa(d_man + extra as i64);

    for iexp in (0..=19).step_by(sample) {
        let man = 10_i64.pow(iexp);

        let n = Dec64::from_mantissa(man);

        group.bench_with_input(BenchmarkId::new("prim-fpdec:64", iexp), &(n, d), |b, i| {
            b.iter(|| i.0 / i.1)
        });
    }
}

// entry
fn criterion_benchmark(c: &mut Criterion) {
    let machine = env::var("MACHINE").unwrap_or_default();
    let sample = env::var("SAMPLE")
        .map(|s| usize::from_str(&s).expect("invalid SAMPLE"))
        .unwrap_or(1);

    let mut group = c.benchmark_group(format!("division:evenly{machine}"));
    bench_bigdecimal(&mut group, sample, false);
    bench_fastnum(&mut group, sample, false);
    bench_rust_decimal(&mut group, sample, false);
    bench_decimax(&mut group, sample, false);
    bench_primitive_fixed_point_decimal(&mut group, sample, false);
    bench_f64(&mut group, sample, false);
    group.finish();

    let mut group = c.benchmark_group(format!("division:non-evenly{machine}"));
    bench_bigdecimal(&mut group, sample, true);
    bench_fastnum(&mut group, sample, true);
    bench_rust_decimal(&mut group, sample, true);
    bench_decimax(&mut group, sample, true);
    bench_primitive_fixed_point_decimal(&mut group, sample, true);
    bench_f64(&mut group, sample, true);
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
