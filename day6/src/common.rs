use std::collections::VecDeque;

pub fn find_marker(input: &str, len: usize) -> usize {
    let mut previous = VecDeque::with_capacity(len);
    for (index, c) in &mut input.char_indices() {
        if let Some(dup) = previous.iter().position(|p| p == &c) {
            previous.drain(0..=dup);
        }

        previous.push_back(c);
        if previous.len() == len {
            return index + 1;
        }
    }

    panic!("marker not found");
}
