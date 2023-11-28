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

pub fn random_value_with_excluded_range_f32(
    range: Range<f32>,
    excluded_values: Vec<Range<f32>>,
) -> f32 {
    let mut value = random_value_f32(generate_seed(), range);

    // while excluded_values
    // .iter()
    // .any(|&excluded_value| (value - excluded_value).abs() <= range)
    // {
    // value = random_value_f32(generate_seed(), minimum_value, maximum_value);
    // }

    value
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

    #[test]
    fn generate_a_random_value_with_excluded_values_f32_case_1() {
        // Given
        let range = 0.0..10.0;
        let excluded_values: Vec<Range<f32>> = vec![0.0..9.0];

        // When
        let actual = random_value_with_excluded_range_f32(range, excluded_values);

        // Then
        assert!(!(0.0..9.0).contains(&actual));
    }

    #[test]
    fn generate_a_random_value_with_excluded_values_f32_case_2() {
        // Given
        let range = 0.0..501.0;
        let excluded_values: Vec<Range<f32>> = vec![0.0..400.0];

        // When
        let actual = random_value_with_excluded_range_f32(range, excluded_values);

        // Then
        assert!(!(0.0..400.0).contains(&actual));
    }
}
