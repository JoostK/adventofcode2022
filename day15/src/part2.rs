use shared::point::Point;
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Sensor {
    pos: Point,
    distance: isize,
    left: Point,
    right: Point,
    top: Point,
    bottom: Point,
}

pub fn parse_sensors(input: &str) -> Vec<Sensor> {
    input.lines().map(parse_sensor).collect()
}

fn parse_sensor(input: &str) -> Sensor {
    let (sensor, beacon) = input.split_once(':').unwrap();

    let pos = parse_coord(sensor);
    let closest_beacon = parse_coord(beacon);
    let dx = (closest_beacon.x - pos.x).abs();
    let dy = (closest_beacon.y - pos.y).abs();
    let distance = dx + dy;
    let perimeter = distance + 1;

    let right = Point::new(pos.x + perimeter, pos.y);
    let bottom = Point::new(pos.x, pos.y + perimeter);
    let left = Point::new(pos.x - perimeter, pos.y);
    let top = Point::new(pos.x, pos.y - perimeter);

    Sensor {
        pos,
        distance,
        right,
        bottom,
        left,
        top,
    }
}

fn parse_coord(data: &str) -> Point {
    let (x, y) = data.split_once(',').unwrap();
    let x = x.rsplit('=').next().unwrap();
    let y = y.rsplit('=').next().unwrap();

    Point::new(x.parse().unwrap(), y.parse().unwrap())
}

impl Sensor {
    pub fn reaches(&self, pt: Point) -> bool {
        let dx = (pt.x - self.pos.x).abs();
        let dy = (pt.y - self.pos.y).abs();
        dx + dy <= self.distance
    }
}

fn intersect_segments(
    down_start: Point,
    down_end: Point,
    up_start: Point,
    up_end: Point,
) -> Option<Point> {
    //     X=0
    // Y=10    down_start  up_end
    //             \         /
    //              \       /
    //               \     /
    //                \   /
    //                 \ /
    //                  X
    //                 / \
    //                /   \
    //               /     \
    //              /       \
    //  Y=0  up_start   down_end

    debug_assert!(down_start.x < down_end.x);
    debug_assert!(down_start.y > down_end.y);
    debug_assert!(up_start.x < up_end.x);
    debug_assert!(up_start.y < up_end.y);

    let c1 = down_start.x + down_start.y;
    let c2 = up_start.x - up_start.y;
    let mut x = c1 + c2;
    if x & 1 == 1 {
        return None;
    }

    // x = (c1 + c2) / 2;
    x >>= 1;
    if x < down_start.x || x > down_end.x || x < up_start.x || x > up_end.x {
        return None;
    }

    let y = c1 - x;
    Some(Point::new(x, y))
}

fn intersect_perimeter(s1: &Sensor, s2: &Sensor) -> [Option<Point>; 8] {
    [
        intersect_segments(s1.bottom, s1.right, s2.left, s2.bottom),
        intersect_segments(s2.bottom, s2.right, s1.left, s1.bottom),
        intersect_segments(s1.bottom, s1.right, s2.top, s2.right),
        intersect_segments(s2.bottom, s2.right, s1.top, s1.right),
        intersect_segments(s1.left, s1.top, s2.left, s2.bottom),
        intersect_segments(s2.left, s2.top, s1.left, s1.bottom),
        intersect_segments(s1.left, s1.top, s2.top, s2.right),
        intersect_segments(s2.left, s2.top, s1.top, s1.right),
    ]
}

fn find_beacon(sensors: &[Sensor], max: isize) -> Option<Point> {
    let mut counts = BTreeMap::new();
    for (i, s1) in sensors.iter().enumerate() {
        for s2 in &sensors[(i + 1)..] {
            let intersections = intersect_perimeter(s1, s2);
            for intersection in intersections.iter().filter_map(|&i| i) {
                if intersection.x < 0
                    || intersection.x > max
                    || intersection.y < 0
                    || intersection.y > max
                {
                    continue;
                }

                if sensors.iter().any(|sensor| sensor.reaches(intersection)) {
                    continue;
                }

                let entry = counts.entry(intersection).or_insert(0);
                if *entry == 3 {
                    return Some(intersection);
                }
                *entry += 1;
            }
        }
    }

    None
}

pub fn run(input: &str, max: isize) -> usize {
    let sensors = parse_sensors(input);

    let beacon = find_beacon(&sensors, max).expect("should find beacon");

    beacon.x as usize * 4_000_000 + beacon.y as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let example = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

        assert_eq!(run(example, 20), 56_000_011);
    }
}
