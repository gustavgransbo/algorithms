use algorithms::merge_sort::merge_sort;
use criterion::{criterion_group, criterion_main, BatchSize, Criterion};

use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};

fn read_test_data(path: &str) -> Result<Vec<i32>, Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut v = Vec::<i32>::new();

    for line in reader.lines() {
        let line = line?;
        let n = line
            .trim()
            .parse()
            .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
        v.push(n);
    }

    Ok(v)
}

fn big_input_one_million(c: &mut Criterion) {
    let data = read_test_data("benches/data/sorting_1M.in").unwrap();
    c.bench_function("Mergesort - Big Input 1M", |b| {
        b.iter_batched_ref(
            || data.clone(),
            |mut data| merge_sort(&mut data),
            BatchSize::LargeInput,
        )
    });
}

fn big_input_ten_thousand(c: &mut Criterion) {
    let data = read_test_data("benches/data/sorting_10K.in").unwrap();
    c.bench_function("Mergesort - Big Input 10K", |b| {
        b.iter_batched_ref(
            || data.clone(),
            |mut data| merge_sort(&mut data),
            BatchSize::SmallInput,
        )
    });
}

criterion_group! {
    name=benches;
    config = Criterion::default().sample_size(30);
    targets = big_input_ten_thousand, big_input_one_million
}
criterion_main!(benches);
