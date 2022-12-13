use crate::common::*;

fn divider_id(items: &Vec<Item>) -> Option<usize> {
    if items.len() != 1 {
        return None;
    }
    let inner = if let Item::List(inner) = &items[0] {
        inner
    } else {
        return None;
    };

    if inner.len() != 1 {
        return None;
    }

    if let Item::Singleton(v) = inner[0] {
        Some(v)
    } else {
        None
    }
}

pub fn run(input: &str) -> usize {
    let mut items: Vec<_> = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(parse_items)
        .chain([
            vec![Item::List(vec![Item::Singleton(2)])],
            vec![Item::List(vec![Item::Singleton(6)])],
        ])
        .collect();

    items.sort_by(compare_items);

    items
        .iter()
        .enumerate()
        .filter_map(|(index, items)| {
            matches!(divider_id(items), Some(2) | Some(6)).then_some(index + 1)
        })
        .product()
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
