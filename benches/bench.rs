use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn bench_round_down(c: &mut Criterion) {
	fn naive_round_down(value: usize, mult: usize) -> usize {
		value / mult * mult
	}

	let mut g = c.benchmark_group("Round Down");

	g.bench_function("round_mult", |b| {
		b.iter(|| {
			black_box(round_mult::down::<usize>(black_box(109), 10));
		});
	});

	g.bench_function("Naive", |b| {
		b.iter(|| {
			black_box(naive_round_down(black_box(109), 10));
		});
	});

	g.finish();
}

criterion_group!(group_round_down, bench_round_down);
criterion_main!(group_round_down);
