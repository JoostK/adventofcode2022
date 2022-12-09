pub fn decode_direction(dir: &str) -> (i32, i32) {
    match dir {
        "L" => (-1, 0),
        "R" => (1, 0),
        "U" => (0, 1),
        "D" => (0, -1),
        _ => panic!("invalid direction {}", dir),
    }
}
