use criterion::{black_box, criterion_group, criterion_main, Criterion};
use round_mult::NonZeroPow2;

pub fn bench_round_down(c: &mut Criterion) {
	let mult32 = NonZeroPow2::<usize>::v32();

	let mut g = c.benchmark_group("Round Down");

	g.bench_function("round_mult 32 NZP", |b| {
		b.iter(|| {
			black_box(round_mult::down(black_box(109), black_box(mult32)));
		});
	});

	g.bench_function("round_mult 32 NZ", |b| {
		b.iter(|| {
			black_box(round_mult::down(
				black_box(109),
				black_box(mult32.get_nonzero()),
			));
		});
	});

	g.finish();
}

pub fn bench_round_up(c: &mut Criterion) {
	let mult32 = NonZeroPow2::<usize>::v32();
	let mut g = c.benchmark_group("Round Up");

	g.bench_function("lib round up NZP", |b| {
		b.iter(|| {
			black_box(round_mult::up(black_box(100usize), black_box(mult32)))
		});
	});

	g.bench_function("std round up NZP", |b| {
		b.iter(|| {
			black_box(black_box(100usize).checked_next_multiple_of(32))
		});
	});
}

criterion_group!(group_round_down, bench_round_down);
criterion_group!(group_round_up, bench_round_up);
criterion_main!(group_round_down, group_round_up);
