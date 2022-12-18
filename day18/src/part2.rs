use shared::iterator::ToArray;

#[derive(Copy, Clone, Eq, PartialEq)]
enum State {
    Pending,
    Lava,
    Air,
}

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
    let mut states = vec![State::Pending; max + 1];
    for &pt in &pts {
        let c = coord(pt);
        states[c] = State::Lava;
    }

    let mut q = Vec::with_capacity(max);
    q.push((minx, miny, minz));

    while let Some(pt @ (x, y, z)) = q.pop() {
        let c = coord(pt);
        if x > minx {
            let dc = c - height;
            if states[dc] == State::Pending {
                states[dc] = State::Air;
                q.push((x - 1, y, z));
            }
        }
        if x < maxx {
            let dc = c + height;
            if states[dc] == State::Pending {
                states[dc] = State::Air;
                q.push((x + 1, y, z));
            }
        }
        if y > miny {
            let dc = c - 1;
            if states[dc] == State::Pending {
                states[dc] = State::Air;
                q.push((x, y - 1, z));
            }
        }
        if y < maxy {
            let dc = c + 1;
            if states[dc] == State::Pending {
                states[dc] = State::Air;
                q.push((x, y + 1, z));
            }
        }
        if z > minz {
            let dc = c - size;
            if states[dc] == State::Pending {
                states[dc] = State::Air;
                q.push((x, y, z - 1));
            }
        }
        if z < maxz {
            let dc = c + size;
            if states[dc] == State::Pending {
                states[dc] = State::Air;
                q.push((x, y, z + 1));
            }
        }
    }

    pts.iter()
        .map(|&pt| {
            let c = coord(pt);

            let yn = (states[c - 1] != State::Air) as u8;
            let yp = (states[c + 1] != State::Air) as u8;
            let xn = (states[c - height] != State::Air) as u8;
            let xp = (states[c + height] != State::Air) as u8;
            let zn = (states[c - size] != State::Air) as u8;
            let zp = (states[c + size] != State::Air) as u8;

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

        assert_eq!(run(example), 58);
    }
}
