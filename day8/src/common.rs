pub struct Forest {
    pub width: usize,
    pub height: usize,
    pub trees: Vec<u8>,
}

impl From<&str> for Forest {
    fn from(input: &str) -> Self {
        let mut trees = Vec::with_capacity(input.len());

        let mut width = 0;
        let mut height = 0;
        for line in input.lines() {
            width = line.len();
            height += 1;
            trees.extend(line.as_bytes().iter().map(|&height| height - b'0'));
        }

        Self {
            width,
            height,
            trees,
        }
    }
}

impl Forest {
    pub fn for_each_interior(&self, mut cb: impl FnMut((usize, usize))) {
        for x in 1..(self.width - 1) {
            for y in 1..(self.height - 1) {
                cb((x, y));
            }
        }
    }

    pub fn tree_height(&self, x: usize, y: usize) -> u8 {
        self.trees[self.width * y + x]
    }
}
