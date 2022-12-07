mod grid;

fn main() {
    let input = include_str!("input.txt");

    let (mut x, mut y, mut dx, mut dy, mut angle) = (0, 0, 0, 0, 0);
    let mut grid = grid::Grid::default();

    for line in input.lines() {
        let (instruction, arg) = line.split_once(' ').unwrap();
        let arg: i32 = arg.parse().unwrap();

        match instruction {
            "draai" => {
                angle = (angle + arg) % 360;

                dx = (angle as f64).to_radians().sin().round() as i32;
                dy = (angle as f64).to_radians().cos().round() as i32;
            }
            "loop" => {
                grid.mark(x, y);
                for _ in 0..arg {
                    x += dx;
                    y += dy;
                    grid.mark(x, y);
                }
            }
            "spring" => {
                x += dx * arg;
                y += dy * arg;
            }
            _ => panic!("unexpected instruction {}", instruction),
        }
    }

    println!("{} + {} = {}\n", x.abs(), y.abs(), x.abs() + y.abs());
    println!("{}", grid);
}
