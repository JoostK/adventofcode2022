use crate::point::Point;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u16)]
pub enum Direction {
    Up = 0xff00,
    Down = 0x0100,
    Left = 0x00ff,
    Right = 0x0001,
}

pub fn move_point(pt: &Point, dir: Direction) -> Point {
    let d = unsafe { std::mem::transmute::<Direction, u16>(dir) };
    let dx = d as i8 as isize;
    let dy = (d >> 8) as i8 as isize;
    Point {
        x: pt.x + dx,
        y: pt.y + dy,
    }
}
