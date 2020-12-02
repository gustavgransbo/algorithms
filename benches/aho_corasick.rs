use algorithms::aho_corasick::PatternFinder;
use criterion::{criterion_group, criterion_main, BatchSize, Criterion};

fn long_text_always_matching(c: &mut Criterion) {
    let text = "a".repeat(100000);
    let pattern_finder = PatternFinder::new(vec![
        String::from("a"),
        String::from("aa"),
        String::from("aaa"),
        String::from("ab"),
    ]);
    c.bench_function("Aho-Corasick - Long text always matching", |b| {
        b.iter_batched_ref(
            || text.clone(),
            |text| pattern_finder.find_patterns(&text),
            BatchSize::SmallInput,
        )
    });
}

fn long_text_often_failing(c: &mut Criterion) {
    let text = "aaaaaaaaab".repeat(10000);
    let pattern_finder = PatternFinder::new(vec![
        String::from("a"),
        String::from("aa"),
        String::from("aaa"),
        String::from("aaaa"),
        String::from("aaaaa"),
        String::from("aaaaaa"),
        String::from("aaaaaaa"),
        String::from("aaaaaaaa"),
        String::from("aaaaaaaaa"),
        String::from("aaaaaaaaaa"), // This pattern will often be reached, but it will never match
    ]);
    c.bench_function("Aho-Corasick - Long text often failing", |b| {
        b.iter_batched_ref(
            || text.clone(),
            |text| pattern_finder.find_patterns(&text),
            BatchSize::SmallInput,
        )
    });
}

criterion_group! {
    name=benches;
    config = Criterion::default().sample_size(30);
    targets = long_text_always_matching, long_text_often_failing
}
criterion_main!(benches);
