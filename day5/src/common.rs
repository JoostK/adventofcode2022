use shared::iterator::ToArray;

pub struct Stacks {
    pub stacks: Vec<Vec<char>>,
}

impl Stacks {
    pub fn parse(input: &str) -> Self {
        let mut lines = input.lines().rev();
        let indices = lines.next().expect("expected indices");
        let amount = (indices.len() + 1) / 4;
        let mut stacks = Vec::from_iter((0..amount).map(|_| Vec::default()));
        for line in lines {
            let mut chars = line.chars();
            for stack in &mut stacks {
                let [_, char, _] = chars.collect_array();
                if char != ' ' {
                    stack.push(char);
                }
                chars.next();
            }
        }
        Stacks { stacks }
    }

    pub fn top(&self) -> String {
        self.stacks
            .iter()
            .map(|stack| stack.last().cloned().unwrap_or(' '))
            .collect()
    }
}

pub struct Instruction {
    pub quantity: usize,
    pub from: usize,
    pub to: usize,
}

pub fn parse_instruction(line: &str) -> Instruction {
    let [_, quantity, _, from, _, to] = line.split(' ').collect_array();

    Instruction {
        quantity: quantity.parse().expect("expected quantity"),
        from: from.parse().expect("expected quantity"),
        to: to.parse().expect("expected quantity"),
    }
}
