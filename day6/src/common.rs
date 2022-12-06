use std::collections::VecDeque;

pub fn find_marker(data: &mut impl Iterator<Item = char>, len: usize) -> usize {
    let mut previous = VecDeque::with_capacity(len);
    for (index, c) in &mut data.enumerate() {
        if let Some(dup) = previous.iter().position(|p| p == &c) {
            previous.drain(0..(dup + 1));
        }

        previous.push_back(c);
        if previous.len() == len {
            return index + 1;
        }
    }

    unreachable!("marker not found");
}
