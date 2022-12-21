pub fn run(input: &str) -> isize {
    let nums: Vec<_> = input
        .lines()
        .map(|line| line.parse::<i16>().unwrap())
        .collect();

    let n = nums.len();
    let mut indices: Vec<_> = (0..n).collect();

    for (index, num) in nums.iter().enumerate() {
        let pos = indices.iter().position(|n| n == &index).unwrap();
        let new_pos = (pos as i16 + num).rem_euclid(n as i16 - 1) as usize;

        if pos < new_pos {
            indices[pos..=new_pos].rotate_left(1);
        } else {
            indices[new_pos..=pos].rotate_right(1);
        }
    }

    let zero = nums.iter().position(|&i| i == 0).unwrap();
    let zero = indices.iter().position(|&i| i == zero).unwrap();
    [1000, 2000, 3000]
        .iter()
        .map(|o| {
            let i = (zero + o) % n as usize;
            nums[indices[i]] as isize
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let example = "1
2
-3
3
-2
0
4";

        assert_eq!(run(example), 3);
    }
}
