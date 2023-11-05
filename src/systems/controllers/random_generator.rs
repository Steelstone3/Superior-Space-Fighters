use rand::{rngs::StdRng, Rng, SeedableRng};

pub fn random_range(seed: u64, minimum:i32, maximum: i32) -> i32 {
    let mut rng = StdRng::seed_from_u64(seed);

    rng.gen_range(minimum..maximum)
}

pub fn generate_seed() -> u64 {
    let mut rng = rand::thread_rng();

    rng.gen_range(u64::MIN..u64::MAX)
}

#[cfg(test)]
mod random_generator_should {
    use super::*;

    #[test]
    fn generate_a_random_range() {
        let seed = 1234;
        let minimum = -20;
        let maximum = 20;
        let expected = -18;

        let actual = random_range(seed, minimum, maximum);

        assert_eq!(expected, actual);
    }
}
