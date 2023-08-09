use criterion::{black_box, criterion_group, criterion_main, Criterion};
use progress_encode::encode;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("encode blank", |b| b.iter(|| encode(black_box(b""))));
    c.bench_function("encode short password", |b| {
        b.iter(|| encode(black_box(b"passw0rd")))
    });
    c.bench_function("encode long string", |b| {
        b.iter(|| {
            encode(black_box(
                b"ThisIsALongerPasswordToSeeHowLengthImpactsThings",
            ))
        })
    });
    c.bench_function("encode very long string", |b| {
        b.iter(|| encode(black_box(include_bytes!("very_large_encode.bin"))))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
