use criterion::{criterion_group, criterion_main, Criterion, BatchSize};
use algorithms::quick_sort::quick_sort;


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

fn big_input_one_million(c: &mut Criterion){
    let data = read_test_data("benches/data/sorting_1M.in").unwrap();
    c.bench_function("Quicksort - Big Input 1M", 
        |b| b.iter_batched_ref(
            || data.clone(),
            |mut data| quick_sort(&mut data),
            BatchSize::LargeInput
        )
    );
}

fn big_input_ten_thousand(c: &mut Criterion){
    let data = read_test_data("benches/data/sorting_10K.in").unwrap();
    c.bench_function("Quicksort - Big Input 10K", 
        |b| b.iter_batched_ref(
            || data.clone(),
            |mut data| quick_sort(&mut data),
            BatchSize::SmallInput
        )
    );
}

fn big_input_ten_thousand_sorted(c: &mut Criterion){
    let data = read_test_data("benches/data/sorting_sorted_10K.in").unwrap();
    c.bench_function("Quicksort - Big Input 10K Sorted", 
        |b| b.iter_batched_ref(
            || data.clone(),
            |mut data| quick_sort(&mut data),
            BatchSize::SmallInput
        )
    );
}

criterion_group!{
    name=benches;
    config = Criterion::default().sample_size(30);
    targets = big_input_ten_thousand, big_input_one_million, big_input_ten_thousand_sorted
}
criterion_main!(benches);