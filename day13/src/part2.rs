use crate::common::*;

pub fn run(input: &str) -> usize {
    let (i2, i6) = input
        .lines()
        .filter(|l| !l.is_empty())
        .fold((1, 2), |(i2, i6), line| {
            if compare_items("[[2]]", line).is_gt() {
                (i2 + 1, i6 + 1)
            } else if compare_items("[[6]]", line).is_gt() {
                (i2, i6 + 1)
            } else {
                (i2, i6)
            }
        });
    i2 * i6
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let example = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

        assert_eq!(run(example), 140);
    }
}
