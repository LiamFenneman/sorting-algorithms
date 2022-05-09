use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::{prelude::SliceRandom, thread_rng};
use sorting::{merge_sort, quick_sort};

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut list = (0..100).collect::<Vec<u32>>();
    list.shuffle(&mut thread_rng());

    let mut group = c.benchmark_group("Sort 100");
    group.bench_with_input("Merge", &list, |b, i| b.iter(|| merge_sort(black_box(i))));
    group.bench_with_input("Quick", &list, |b, i| b.iter(|| quick_sort(black_box(i))));
    group.finish();

    let mut list = (0..5000).collect::<Vec<u32>>();
    list.shuffle(&mut thread_rng());

    let mut group = c.benchmark_group("Sort 5k");
    group.bench_with_input("Merge", &list, |b, i| b.iter(|| merge_sort(black_box(i))));
    group.bench_with_input("Quick", &list, |b, i| b.iter(|| quick_sort(black_box(i))));
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
