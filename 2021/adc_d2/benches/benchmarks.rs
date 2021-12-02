use adc_d2::util::get_input_lines_without_buffer;
use adc_d2::{get_solution_for_part_one, get_solution_for_part_two};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn day_two_benchmark(c: &mut Criterion) {
    c.bench_function("day_two", |b| {
        b.iter(|| {
            let input_lines = get_input_lines_without_buffer();
            get_solution_for_part_one(black_box(&input_lines));
            get_solution_for_part_two(black_box(&input_lines))
        })
    });
}

pub fn day_two_part_one_benchmark(c: &mut Criterion) {
    c.bench_function("day_two part 1", |b| {
        b.iter(|| {
            let input_lines = get_input_lines_without_buffer();
            get_solution_for_part_one(black_box(&input_lines));
        })
    });
}

pub fn day_two_part_two_benchmark(c: &mut Criterion) {
    c.bench_function("day_two part 2", |b| {
        b.iter(|| {
            let input_lines = get_input_lines_without_buffer();
            get_solution_for_part_two(black_box(&input_lines));
        })
    });
}

criterion_group!(
    benches,
    day_two_benchmark,
    day_two_part_one_benchmark,
    day_two_part_two_benchmark
);
criterion_main!(benches);
