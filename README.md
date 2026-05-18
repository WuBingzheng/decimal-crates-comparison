Compare and benchmark some decimal crates:

- `bigdecimal` Arbitrary-precision, but slow;
- `fastnum` Arbitrary precision in complilation time;
- `rust_decimal` Most popular;
- `decimax` Similar with `rust_decimal` but faster;
- `primitive_fixed_point_decimal` Fixed-point.

The benchmark results are under [charts/].

[This artical](https://wubingzheng.github.io/en/Decimal-Crates-Comparison.html)
([Chianese version](https://wubingzheng.github.io/zh/Decimal-Crates-Comparison.html))
introduces the benchmark plan and analyzes the results.


# Environment

Versions:

- Rust: `cargo 1.93.0 (083ac5135 2025-12-15)`
- `criterion`: `0.7`

Machines:

- Ubuntu 22.04 @AMD EPYC 9754
- Ubuntu 16.04 @Intel Xeon, 2500 MHZ
- MacOS 13.5 @Apple M1

See [Cargo.toml] for crates version.

The results varied at different Machines.
You are welcome to run the benchmark on your own computer.


# Usage

Run the benchmark:

```bash
git clone https://github.com/WuBingzheng/decimal-crates-comparison.git
cd decimal-crates-comparison
cargo bench
open target/criterion/report/index.html
```

The issue with `cargo bench` is that the color differences in the generated
chart are not very distinct. You can use `cargo criterion` instead, but it
requires installation:

```bash
cargo install cargo-criterion # install
cargo criterion
open target/criterion/reports/index.html # different path, better charts
```

Running all the benchmark takes several hours. You can specify a subset:

```bash
cargo criterion addition:pure/rust_decimal # run only `addition:pure` for `rust_decimal`
```

You can also use the `SAMPLE` environment variable to set the sampling rate:

```bash
SAMPLE=10 cargo criterion # run 1/10 tests for each case
```
