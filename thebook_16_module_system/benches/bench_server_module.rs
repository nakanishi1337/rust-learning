use criterion::{black_box, criterion_group, criterion_main, Criterion};
use thebook_16_module_system::modules::server_module;

fn bench_test_access(c: &mut Criterion) {
    c.bench_function("test_access", |b| {
        b.iter(|| server_module::test_access())
    });
}

criterion_group!(benches, bench_test_access);
criterion_main!(benches);