use rand::{rngs::StdRng, Rng, SeedableRng};

pub fn random_value_i32(seed: u64, minimum: i32, maximum: i32) -> i32 {
    let mut rng = StdRng::seed_from_u64(seed);

    rng.gen_range(minimum..maximum)
}

pub fn random_value_f32(seed: u64, minimum: f32, maximum: f32) -> f32 {
    let mut rng = StdRng::seed_from_u64(seed);

    rng.gen_range(minimum..maximum)
}

pub fn generate_seed() -> u64 {
    let mut rng = rand::thread_rng();

    rng.gen_range(u64::MIN..u64::MAX)
}

pub fn random_value_with_excluded_range_f32(
    seed: u64,
    minimum_value: f32,
    maximum_value: f32,
    excluded_values: Vec<f32>,
    range: f32,
) -> f32 {
    let mut value = random_value_f32(seed, minimum_value, maximum_value);

    while excluded_values.contains(&value) {
        value = random_value_f32(generate_seed(), minimum_value, maximum_value);
    }

    value
}

#[cfg(test)]
mod random_generator_should {
    use super::*;

    #[test]
    fn generate_a_random_value_i32() {
        // Given
        let seed = 1234;
        let minimum = -20;
        let maximum = 20;
        let expected = -18;

        // When
        let actual = random_value_i32(seed, minimum, maximum);

        // Then
        assert_eq!(expected, actual);
    }

    #[test]
    fn generate_a_random_value_f32() {
        // Given
        let seed = 1234;
        let minimum = -20.0;
        let maximum = 20.0;
        let expected = -17.736874;

        // When
        let actual = random_value_f32(seed, minimum, maximum);

        // Then
        assert_eq!(expected, actual);
    }

    #[test]
    fn generate_a_random_value_with_excluded_values_f32_case_1() {
        // Given
        let seed = 1234;
        let minimum = 0.0;
        let maximum = 10.0;
        let excluded_values: Vec<f32> = vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];

        // When
        let actual =
            random_value_with_excluded_range_f32(seed, minimum, maximum, excluded_values, 1.0);

        // Then
        assert_eq!(0.0, actual);
        assert!(!(0.0..9.0).contains(&actual));
    }

    #[test]
    fn generate_a_random_value_with_excluded_values_f32_case_2() {
        // Given
        let seed = 1234;
        let minimum = 0.0;
        let maximum = 501.0;
        let excluded_values: Vec<f32> = vec![0.0, 100.0, 200.0, 300.0, 400.0];

        // When
        let actual =
            random_value_with_excluded_range_f32(seed, minimum, maximum, excluded_values, 100.0);

        // Then
        assert_eq!(0.0, actual);
        assert!(!(0.0..400.0).contains(&actual));
    }
}
