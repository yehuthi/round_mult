# round_mult

A tiny library to round a number up or down to a multiplier.

# Usage

The library has two functions:
- `round_mult::`[`up`]
- `round_mult::`[`down`]

They both take a value and a multiplier and round the value down or up to the multiplier respectively.

E.g.
```rust
assert_eq!(
    round_mult::down(109, 10),
    100
);

assert_eq!(
    round_mult::up(101, 10),
    110
);
```

# Example: SIMD

The main motivation for this library is SIMD processing. Specifically when the length of data isn't a multiple of the SIMD lanes count, which means you will have a remainder of data to process without SIMD.

```ignore
fn f(data: &[u8]) {
	type Simd = std::simd::u8x32; // or whichever

	let mut i = 0;

	while i < round_mult::down(data.len(), Simd::LANES) {
		let data = Simd::from_slice(s[i..]);
		// SIMD process…
		i += Simd::LANES;
	}
	while i < len {
		// remainder process…
	}
}
```

# Features

## num-traits

This feature makes the library work on traits from the [`num-traits`](https://crates.io/crates/num-traits) crate.
It is off by default.
