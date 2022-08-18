use std::num::NonZeroUsize;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use round_mult::NonZeroPow2;

const MULT32: NonZeroPow2<usize> =
	unsafe { NonZeroPow2::from_nonzero_unchecked(NonZeroUsize::new_unchecked(32)) };

pub fn bench_round_down(c: &mut Criterion) {
	let mut g = c.benchmark_group("Round Down");

	g.bench_function("round_mult 32 NZP", |b| {
		b.iter(|| {
			black_box(round_mult::down(black_box(109), black_box(MULT32)));
		});
	});

	g.bench_function("round_mult 32 NZ", |b| {
		b.iter(|| {
			black_box(round_mult::down(
				black_box(109),
				black_box(MULT32.get_nonzero()),
			));
		});
	});

	g.finish();
}

criterion_group!(group_round_down, bench_round_down);
criterion_main!(group_round_down);
