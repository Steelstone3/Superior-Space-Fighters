use std::ops::Range;

use rand::{rngs::StdRng, Rng, SeedableRng};

pub fn random_value_i32(seed: u64, range: Range<i32>) -> i32 {
    let mut rng = StdRng::seed_from_u64(seed);

    rng.gen_range(range.start..range.end)
}

pub fn random_value_f32(seed: u64, range: Range<f32>) -> f32 {
    let mut rng = StdRng::seed_from_u64(seed);

    rng.gen_range(range.start..range.end)
}

pub fn generate_seed() -> u64 {
    let mut rng = rand::thread_rng();

    rng.gen_range(u64::MIN..u64::MAX)
}

#[cfg(test)]
mod random_generator_should {
    use super::*;

    #[test]
    fn generate_a_random_value_i32() {
        // Given
        let seed = 1234;
        let range = -20..20;
        let expected = -18;

        // When
        let actual = random_value_i32(seed, range);

        // Then
        assert_eq!(expected, actual);
    }

    #[test]
    fn generate_a_random_value_f32() {
        // Given
        let seed = 1234;
        let range = -20.0..20.0;
        let expected = -17.736874;

        // When
        let actual = random_value_f32(seed, range);

        // Then
        assert_eq!(expected, actual);
    }
}
