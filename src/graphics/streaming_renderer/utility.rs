/// Returns the smallest power of two greater than the provided value.
pub(crate) fn round_to_power_of_two(value: usize) -> usize {
    (value as f32).log2().ceil().exp2().round() as usize
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn power_of_two_should_round_up() {
        assert_eq!(round_to_power_of_two(1), 1);
        assert_eq!(round_to_power_of_two(2), 2);
        assert_eq!(round_to_power_of_two(3), 4);
        assert_eq!(round_to_power_of_two(6), 8);
        assert_eq!(round_to_power_of_two(9), 16);
        assert_eq!(round_to_power_of_two(20), 32);
        assert_eq!(round_to_power_of_two(50), 64);
        assert_eq!(round_to_power_of_two(93), 128);
        assert_eq!(round_to_power_of_two(200), 256);
        assert_eq!(round_to_power_of_two(500), 512);
        assert_eq!(round_to_power_of_two(10_000), 16384);
    }
}
