pub fn parse_u8_slice_to_i64(slice: &[u8]) -> i64 {
    slice
        .iter()
        .map(|&c| (c - b'0') as i64)
        .reduce(|acc, next| acc * 10 + next)
        .unwrap()
}
