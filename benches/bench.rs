use std::num::NonZeroUsize;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

const MULT: NonZeroUsize = unsafe { NonZeroUsize::new_unchecked(10) };

pub fn bench_round_down(c: &mut Criterion) {
	fn naive_round_down(value: usize, mult: usize) -> usize {
		value / mult * mult
	}

	let mut g = c.benchmark_group("Round Down");

	g.bench_function("round_mult", |b| {
		b.iter(|| {
			black_box(round_mult::down::<usize>(black_box(109), MULT));
		});
	});

	g.bench_function("Naive", |b| {
		b.iter(|| {
			black_box(naive_round_down(black_box(109), MULT.get()));
		});
	});

	g.finish();
}

pub fn bench_round_up(c: &mut Criterion) {
	fn naive_round_up_1(value: usize, mult: usize) -> usize {
		(value / mult * mult) + mult
	}

	fn naive_round_up_2(value: usize, mult: usize) -> usize {
		((value + mult - 1) / mult) * mult
	}

	// https://github.com/rust-lang/rust/pull/88582/commits/727a4fc7e3f836938dfeb4a2ab237cfca612222d#diff-dd440fe33121a785308d5cde98a1ab79b0b285d27bb29eaa9800e180870e16a6R1848
	pub fn std_round_up(lhs: usize, rhs: usize) -> usize {
		let r = lhs % rhs;

		if r == 0 {
			lhs
		} else {
			lhs + (rhs - r)
		}
	}

	let mut g = c.benchmark_group("Round Up");

	g.bench_function("round_mult", |b| {
		b.iter(|| {
			black_box(round_mult::up::<usize>(black_box(109), MULT));
		});
	});

	g.bench_function("std", |b| {
		b.iter(|| {
			black_box(std_round_up(black_box(109), MULT.get()));
		});
	});

	g.bench_function("Naive #1", |b| {
		b.iter(|| {
			black_box(naive_round_up_1(black_box(109), MULT.get()));
		});
	});

	g.bench_function("Naive #2", |b| {
		b.iter(|| {
			black_box(naive_round_up_2(black_box(109), MULT.get()));
		});
	});

	g.finish();
}

criterion_group!(group_round_up, bench_round_up);
criterion_group!(group_round_down, bench_round_down);
criterion_main!(group_round_down, group_round_up);
