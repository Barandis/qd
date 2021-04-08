// Copyright (c) 2021 Thomas J. Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use criterion::{criterion_group, criterion_main, Criterion};
use qd::Double;

pub fn lntaylor(c: &mut Criterion) {
    c.bench_function("ln (Taylor)", |b| b.iter(|| Double::PI.ln()));
}

pub fn lnagm(c: &mut Criterion) {
    c.bench_function("ln (AGM)", |b| b.iter(|| Double::PI.lnagm()));
}

criterion_group!(benches, lntaylor, lnagm);
criterion_main!(benches);
