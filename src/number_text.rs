fn parse_num(n: u32) -> Vec<(u32, u32)> {
    let n = n.to_string();
    let l = n.len();
    n.chars()
        .enumerate()
        .map(|(i, c)| {
            (
                (10u32.pow((l - i - 1) as u32)),
                c.to_digit(10).expect("This should never happen"),
            )
        })
        .collect::<Vec<(u32, u32)>>()
}

enum PlaceValue {
    _1,
    _10,
    _100,
    _1000,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_num() {
        assert_eq!(parse_num(123), vec![(100, 1), (10, 2), (1, 3)]);
    }
}
