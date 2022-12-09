pub fn decode_direction(dir: &str) -> (isize, isize) {
    match dir {
        "L" => (-1, 0),
        "R" => (1, 0),
        "U" => (0, 1),
        "D" => (0, -1),
        _ => panic!("invalid direction {}", dir),
    }
}
