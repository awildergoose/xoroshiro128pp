#![allow(clippy::missing_panics_doc)]
#![no_std]

pub struct Xoroshiro128PP {
    pub seed_lo: i64,
    pub seed_hi: i64,
}

impl Xoroshiro128PP {
    #[must_use]
    pub const fn new(seed_lo: i64, seed_hi: i64) -> Self {
        let mut lo = seed_lo;
        let mut hi = seed_hi;

        if (lo | hi) == 0 {
            lo = -7_046_029_254_386_353_131;
            hi = 7_640_891_576_956_012_809;
        }

        Self {
            seed_lo: lo,
            seed_hi: hi,
        }
    }

    #[must_use]
    pub const fn from_seed(seed: i64) -> Self {
        let mut this = Self::new(0, 0);
        this.set_seed(seed);
        this
    }

    pub const fn set_seed(&mut self, seed: i64) {
        const fn mix_stafford_13(l: i64) -> i64 {
            let mut value = l;
            value = (value ^ (value.cast_unsigned() >> 30).cast_signed())
                .wrapping_mul(-4_658_895_280_553_007_687);
            value = (value ^ (value.cast_unsigned() >> 27).cast_signed())
                .wrapping_mul(-7_723_592_293_110_705_685);
            value ^ value >> 31
        }

        // unmixed result
        let unmixed_lo = seed ^ 7_640_891_576_956_012_809;
        let unmixed_hi = unmixed_lo.wrapping_add(-7_046_029_254_386_353_131);

        // mix result
        let lo = mix_stafford_13(unmixed_lo);
        let hi = mix_stafford_13(unmixed_hi);

        self.seed_lo = lo;
        self.seed_hi = hi;
    }

    #[must_use]
    pub const fn next_long(&mut self) -> i64 {
        let lo = self.seed_lo;
        let mut hi = self.seed_hi;
        let result = lo.wrapping_add(hi).rotate_left(17).wrapping_add(lo);
        hi ^= lo;
        self.seed_lo = lo.rotate_left(49) ^ hi ^ hi << 21;
        self.seed_hi = hi.rotate_left(28);
        result
    }

    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub const fn next_int(&mut self) -> i32 {
        self.next_long() as i32
    }

    #[must_use]
    pub fn next_int_bounded(&mut self, bound: u32) -> i32 {
        assert!(bound > 0, "Bound must be positive");

        let mut val = u64::from(self.next_int().cast_unsigned());
        let mut m = val.wrapping_mul(u64::from(bound));
        let mut n = (m & 0xffff_ffff) as u32;

        if n < bound {
            let j = bound.wrapping_neg() % bound;

            while n < j {
                val = u64::from(self.next_int().cast_unsigned());
                m = val.wrapping_mul(u64::from(bound));
                n = (m & 0xffff_ffff) as u32;
            }
        }

        let result = m >> 32;
        result as i32
    }

    pub fn next_int_bounded_with_origin(&mut self, origin: i32, bound: i32) -> i32 {
        assert!(origin < bound, "bound - origin is not positive");
        origin + self.next_int_bounded((bound - origin).try_into().unwrap())
    }

    #[must_use]
    pub const fn next_bits(&mut self, i: i64) -> u64 {
        self.next_long().cast_unsigned() >> (64 - i)
    }

    #[must_use]
    pub const fn next_bool(&mut self) -> bool {
        self.next_long() & 1 != 0
    }

    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub const fn next_float(&mut self) -> f32 {
        self.next_bits(24) as f32 * 1.0 / (1u32 << 24) as f32
    }

    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub const fn next_double(&mut self) -> f64 {
        self.next_bits(53) as f64 * 1.0 / (1u64 << 53) as f64
    }
}

#[cfg(test)]
mod test_values;

#[cfg(test)]
mod tests {
    use super::*;
    use test_values::*;

    #[test]
    fn test_long() {
        let mut x = Xoroshiro128PP::new(0, 0);
        assert_eq!(x.seed_lo, -7_046_029_254_386_353_131);
        assert_eq!(x.seed_hi, 7_640_891_576_956_012_809);

        x = Xoroshiro128PP::new(1, 0);
        assert_eq!(x.seed_lo, 1);
        assert_eq!(x.seed_hi, 0);

        for v in EXPECTED_1_0_LONG {
            assert_eq!(x.next_long(), v);
        }
    }

    #[test]
    fn test_int() {
        let mut x = Xoroshiro128PP::new(1, 0);
        assert_eq!(x.seed_lo, 1);
        assert_eq!(x.seed_hi, 0);

        for v in EXPECTED_1_0_INT {
            assert_eq!(x.next_int(), v);
        }
    }

    #[test]
    fn test_int_bounded() {
        let mut x = Xoroshiro128PP::new(1, 0);
        assert_eq!(x.seed_lo, 1);
        assert_eq!(x.seed_hi, 0);

        for v in EXPECTED_1_0_INT_BOUNDED {
            assert_eq!(x.next_int_bounded(100), v);
        }
    }

    #[test]
    fn test_int_bounded_with_origin() {
        let mut x = Xoroshiro128PP::new(1, 0);
        assert_eq!(x.seed_lo, 1);
        assert_eq!(x.seed_hi, 0);

        for v in EXPECTED_1_0_INT_BOUNDED_WITH_ORIGIN {
            assert_eq!(x.next_int_bounded_with_origin(50, 100), v);
        }
    }

    #[test]
    fn test_bool() {
        let mut x = Xoroshiro128PP::new(1, 0);
        assert_eq!(x.seed_lo, 1);
        assert_eq!(x.seed_hi, 0);

        for v in EXPECTED_1_0_BOOL {
            assert_eq!(x.next_bool(), v);
        }
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn test_float() {
        let mut x = Xoroshiro128PP::new(1, 0);
        assert_eq!(x.seed_lo, 1);
        assert_eq!(x.seed_hi, 0);

        for v in EXPECTED_1_0_FLOAT {
            assert_eq!(x.next_float(), v);
        }
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn test_double() {
        let mut x = Xoroshiro128PP::new(1, 0);
        assert_eq!(x.seed_lo, 1);
        assert_eq!(x.seed_hi, 0);

        for v in EXPECTED_1_0_DOUBLE {
            assert_eq!(x.next_double(), v);
        }
    }

    #[test]
    fn test_seed() {
        let x = Xoroshiro128PP::from_seed(1);
        assert_eq!(x.seed_hi, 1_927_618_558_350_093_866);
        assert_eq!(x.seed_lo, 5_272_463_233_947_570_727);
    }
}
