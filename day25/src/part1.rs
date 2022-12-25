fn decode(d: u8) -> isize {
    match d {
        b'2' => 2,
        b'1' => 1,
        b'0' => 0,
        b'-' => -1,
        b'=' => -2,
        _ => panic!("invalid SNAFU {}", d as char),
    }
}

fn from_snafu(input: &str) -> isize {
    let mut sum = 0;
    let mut multiplier = 1;
    for v in input.bytes().rev() {
        let v = decode(v);
        sum += v * multiplier;
        multiplier *= 5;
    }
    sum
}

fn encode(d: isize) -> char {
    match d {
        2 => '2',
        1 => '1',
        0 => '0',
        -1 => '-',
        -2 => '=',
        _ => panic!("invalid value {d}"),
    }
}

fn to_snafu(input: isize) -> String {
    let mut i = input;
    let mut res = String::new();
    while i != 0 {
        let v = (i + 2) % 5 - 2;
        res.insert(0, encode(v));
        i = (i + 2) / 5;
    }

    res
}

pub fn run(input: &str) -> String {
    to_snafu(input.lines().map(from_snafu).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let example = "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";

        assert_eq!(run(example), "2=-1=0");
    }

    #[test]
    fn snafu() {
        assert_eq!(from_snafu("1121-1110-1=0"), 314_159_265);
        assert_eq!(to_snafu(-2), "=");
        assert_eq!(to_snafu(-1), "-");
        assert_eq!(to_snafu(3), "1=");
        assert_eq!(to_snafu(4), "1-");
        assert_eq!(to_snafu(1747), "1=-0-2");
        assert_eq!(to_snafu(314_159_265), "1121-1110-1=0");
    }
}
