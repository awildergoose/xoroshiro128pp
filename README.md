# Xoroshiro128PP

This library mimics Minecraft: Java Edition's Xoroshiro128++ implementation, derived from `XoroshiroRandomSource` and `Xoroshiro128PlusPlus`.

## no_std support

This library is _fully_ compatible with no_std, no need to set any feature flags at all.

## Usage

```rs
let mut rng = Xoroshiro128PP::from_seed(1234567890);
// or manually assign the low and high values
rng = Xoroshiro128PP::new(1234, 5678); // low, high

rng.next_int(); // 0..i32::MAX
rng.next_int_bounded(10); // 0..10
rng.next_int_bounded_with_origin(15, 20); // 15..20
rng.next_long(); // 0..i64::MAX
rng.next_bits(10);
rng.next_bool();
rng.next_float(); // 0..f32::MAX
rng.next_double(); // 0..f64::MAX

```
