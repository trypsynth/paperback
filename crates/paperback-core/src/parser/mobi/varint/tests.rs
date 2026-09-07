use rstest::rstest;

use super::*;

#[rstest]
#[case(&[0x81], 0, (1, 1))]
#[case(&[0x01, 0x82], 0, (130, 2))]
#[case(&[0xFF, 0x81], 1, (1, 2))]
#[case(&[0x00, 0x00, 0x80], 0, (0, 3))]
fn decodes_variable_width_integers(#[case] data: &[u8], #[case] start: usize, #[case] expected: (usize, usize)) {
	assert_eq!(decode_vwi(data, start), expected);
}

#[test]
fn decoding_past_the_end_of_the_data_returns_zero_without_advancing() {
	assert_eq!(decode_vwi(&[0x81], 1), (0, 1));
}

#[rstest]
#[case("0", 0)]
#[case("1", 1)]
#[case("a", 10)]
#[case("A", 10)]
#[case("10", 32)]
#[case("", 0)]
// 'z' is not a valid base-32 digit (radix 32 only covers 0-9, a-v); to_digit returns None,
// which base32_decode treats as a zero digit rather than an error.
#[case("z", 0)]
fn decodes_base32_strings(#[case] input: &str, #[case] expected: usize) {
	assert_eq!(base32_decode(input), expected);
}
