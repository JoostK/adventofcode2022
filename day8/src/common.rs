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

    pub fn left_of(&self, x: usize) -> impl DoubleEndedIterator<Item = usize> {
        (0..x).rev()
    }

    pub fn right_of(&self, x: usize) -> impl DoubleEndedIterator<Item = usize> {
        (x + 1)..self.width
    }

    pub fn above(&self, y: usize) -> impl DoubleEndedIterator<Item = usize> {
        (0..y).rev()
    }

    pub fn below(&self, y: usize) -> impl DoubleEndedIterator<Item = usize> {
        (y + 1)..self.height
    }
}
