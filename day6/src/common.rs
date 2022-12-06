pub fn find_marker(input: &str, len: usize) -> usize {
    (len..input.len())
        .find(|&index| all_unique(&input[(index - len)..index]))
        .expect("marker not found")
}

fn all_unique(data: &str) -> bool {
    data.chars()
        .map(|c| 1 << (c as u32 - 'a' as u32))
        .try_fold(0u32, |bits, bit| (bits & bit == 0).then_some(bits | bit))
        .is_some()
}
