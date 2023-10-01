use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(_c: &mut Criterion) {}

criterion_group! {
    name = benches;
    config = Criterion::default(); //.measurement_time(Duration::from_secs(10));
    targets = criterion_benchmark
}
criterion_main!(benches);
