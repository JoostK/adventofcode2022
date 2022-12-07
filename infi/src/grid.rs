use std::collections::BTreeSet;
use std::fmt::{Display, Formatter, Write};

#[derive(Default)]
pub struct Grid {
    coords: BTreeSet<(i32, i32)>,
    min: (i32, i32),
    max: (i32, i32),
}

impl Grid {
    pub fn mark(&mut self, x: i32, y: i32) {
        self.min = (self.min.0.min(x), self.min.1.min(y));
        self.max = (self.max.0.max(x), self.max.1.max(y));
        self.coords.insert((x, y));
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in (self.min.1..=self.max.1).rev() {
            for x in self.min.0..=self.max.0 {
                if self.coords.contains(&(x, y)) {
                    f.write_str("##")?;
                } else {
                    f.write_str("  ")?;
                }
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}
