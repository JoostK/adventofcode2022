use crate::common::*;
use shared::iterator::ToArray;

pub fn run(input: &str) -> usize {
    input
        .split("\n\n")
        .enumerate()
        .filter_map(|(index, line)| {
            let [p1, p2] = line.lines().collect_array();
            compare_items(p1, p2).is_lt().then_some(index + 1)
        })
        .sum()
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

        assert_eq!(run(example), 13);
    }
}
