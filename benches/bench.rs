#[macro_use]
extern crate criterion;
extern crate lipmaa_link;
extern crate rand;

use criterion::black_box;
use criterion::Criterion;
use lipmaa_link::lipmaa;
use rand::random;

fn recurse(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        n => recurse(lipmaa(n)),
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("walk_from_max_to_min", |b| {
        b.iter(|| recurse(black_box(core::u32::MAX)))
    });
    c.bench_function("walk_from_rand_to_min", |b| {
        b.iter(|| recurse(black_box(random())))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
