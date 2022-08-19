# round_mult [<img src="https://img.shields.io/crates/v/round_mult" align="right" />](https://crates.io/crates/round_mult) [<img src="https://img.shields.io/docsrs/round_mult" align="right" />](https://docs.rs/round_mult)

A tiny library to round a number up or down to a multiplier.

# Usage

The library has two functions:
- `round_mult::`[`up`]
- `round_mult::`[`down`]

They both take a value and a multiplier and round the value down or up to the multiplier respectively.

## Multiplier

There are two kind of multipliers:
- [`NonZeroPow2`] When the multiplier is a power of two, it can be calculated faster. Prefer it when possible.
- [`std::num::NonZeroU_`](https://doc.rust-lang.org/std/num/index.html#:~:text=to%20equal%20zero.-,NonZeroU8,An%20integer%20that%20is%20known%20not%20to%20equal%20zero.,-ParseFloatError) for any multiplier value.

# Example

E.g.
```rust
use std::num::NonZeroUsize;
use round_mult::NonZeroPow2;

assert_eq!(
	round_mult::down(70usize, NonZeroPow2::v32()),
	64
);

// These two are semantically equivalent:
assert_eq!(
	round_mult::down(70usize, NonZeroPow2::new(32).unwrap()),
	round_mult::down(70usize, NonZeroUsize::new(32).unwrap()),
);
// but NonZeroPow2 (the first parameter) is faster.

// However, it can't be used when the multiplier isn't a power of two.
// In that case use a NonZeroU_ type:
assert_eq!(
    round_mult::down(109usize, NonZeroUsize::new(10).unwrap()),
    100
);
assert_eq!(
    round_mult::up(101usize, NonZeroUsize::new(10).unwrap()),
    Some(110)
);
```

# Example: SIMD

The main motivation for this library is SIMD processing. Specifically when the length of data isn't a multiple of the SIMD lanes count, which means you will have a remainder of data to process without SIMD.

```rust
use round_mult::NonZeroPow2;

fn f(data: &[u8]) {
	// for this example, assume std::simd::u8x32 is used.
	let lanes = NonZeroPow2::v32();

	let mut i = 0;

	while i < round_mult::down(data.len(), lanes) {
		// SIMD process…
		// let data = Simd::from_slice(s[i..]);
		// etc. etc.
		i += lanes.get();
	}
	while i < data.len() {
		// remainder process…
		i += 1;
	}
}
```
