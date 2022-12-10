use std::fmt::Write;

const CRT_WIDTH: usize = 40;
const CRT_HEIGHT: usize = 6;

pub fn run(input: &str) -> String {
    let mut cycle = 0;
    let mut register_x = 1;

    let mut crt = String::with_capacity((CRT_WIDTH + 1) * CRT_HEIGHT);

    let mut tick = |cycle: &mut isize, register_x: isize| {
        *cycle += 1;

        let column = (*cycle - 1) % CRT_WIDTH as isize;
        if column == 0 {
            writeln!(crt).expect("CRT display not exceeded");
        }

        if register_x - 1 <= column && column <= register_x + 1 {
            write!(crt, "#").expect("CRT display not exceeded");
        } else {
            write!(crt, ".").expect("CRT display not exceeded");
        }
    };

    for line in input.lines() {
        if line.starts_with("noop") {
            tick(&mut cycle, register_x);

            continue;
        }
        let operand: isize = line[5..].parse().unwrap();

        for _ in 0..2 {
            tick(&mut cycle, register_x);
        }

        register_x += operand;
    }

    crt
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let example = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

        assert_eq!(
            run(example),
            "
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
        );
    }
}
