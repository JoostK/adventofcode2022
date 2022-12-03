use std::collections::HashSet;

fn to_priority(letter: char) -> u8 {
    if letter.is_ascii_lowercase() {
        letter as u8 - b'a' + 1
    } else {
        letter as u8 - b'A' + 1 + 26
    }
}

pub fn parse_set(compartment: &str) -> HashSet<u8> {
    HashSet::from_iter(compartment.chars().map(to_priority))
}
