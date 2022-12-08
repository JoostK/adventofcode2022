pub fn run(input: &str) -> usize {
    input.lines().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let example = "";

        assert_eq!(run(example), 0);
    }
}
