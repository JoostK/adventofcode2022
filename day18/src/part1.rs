use shared::iterator::ToArray;

pub fn run(input: &str) -> usize {
    let (mut minx, mut maxx) = (isize::MAX, isize::MIN);
    let (mut miny, mut maxy) = (isize::MAX, isize::MIN);
    let (mut minz, mut maxz) = (isize::MAX, isize::MIN);

    let pts: Vec<_> = input
        .lines()
        .map(|line| {
            let [x, y, z] = line.split(',').collect_array();

            let x: isize = x.parse().unwrap();
            let y: isize = y.parse().unwrap();
            let z: isize = z.parse().unwrap();

            minx = minx.min(x - 1);
            maxx = maxx.max(x + 1);
            miny = miny.min(y - 1);
            maxy = maxy.max(y + 1);
            minz = minz.min(z - 1);
            maxz = maxz.max(z + 1);

            (x, y, z)
        })
        .collect();

    let width = (maxx - minx + 1) as usize;
    let height = (maxy - miny + 1) as usize;
    let size = width * height;

    let coord = |(x, y, z): (isize, isize, isize)| -> usize {
        (z - minz) as usize * size + (x - minx) as usize * height + (y - miny) as usize
    };

    let max = coord((maxx, maxy, maxz));
    let mut lava = vec![false; max + 1];
    for &pt in &pts {
        let c = coord(pt);
        lava[c] = true;
    }

    pts.iter()
        .map(|&pt| {
            let c = coord(pt);

            let yn = lava[c - 1] as u8;
            let yp = lava[c + 1] as u8;
            let xn = lava[c - height] as u8;
            let xp = lava[c + height] as u8;
            let zn = lava[c - size] as u8;
            let zp = lava[c + size] as u8;

            (6 - yn - yp - xn - xp - zn - zp) as usize
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let example = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

        assert_eq!(run(example), 64);
    }
}
