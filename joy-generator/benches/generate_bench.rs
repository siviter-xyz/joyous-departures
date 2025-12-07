use criterion::{black_box, criterion_group, criterion_main, Criterion};
use joy_generator::{generate_goodbye, CoreGoodbyeOptions};

fn bench_generate_message(c: &mut Criterion) {
    let options = CoreGoodbyeOptions::default();

    c.bench_function("generate_goodbye", |b| {
        b.iter(|| black_box(generate_goodbye(black_box(&options)).unwrap()));
    });
}

fn bench_generate_with_custom_name(c: &mut Criterion) {
    let mut options = CoreGoodbyeOptions::default();
    options
        .template_args
        .insert("name".to_string(), "Alice".to_string());

    c.bench_function("generate_goodbye_with_custom_name", |b| {
        b.iter(|| black_box(generate_goodbye(black_box(&options)).unwrap()));
    });
}

fn bench_corpus_loading(c: &mut Criterion) {
    c.bench_function("load_corpus", |b| {
        b.iter(|| black_box(joy_generator::corpus::load_corpus().unwrap()));
    });
}

criterion_group!(
    benches,
    bench_generate_message,
    bench_generate_with_custom_name,
    bench_corpus_loading
);
criterion_main!(benches);

