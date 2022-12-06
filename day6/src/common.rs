pub fn find_marker(input: &str, len: usize) -> usize {
    for index in len..input.len() {
        if all_unique(&input[(index - len)..index]) {
            return index;
        }
    }

    panic!("marker not found");
}

fn all_unique(data: &str) -> bool {
    let mut bits = 0u32;
    for c in data.chars() {
        let bit = 1 << (c as u32 - 'a' as u32);
        if bits & bit != 0 {
            return false;
        }
        bits |= bit;
    }
    true
}
