use crate::common::*;
use shared::iterator::ToArray;
use std::collections::VecDeque;

fn move_crates(stacks: &mut Stacks, instruction: &Instruction) {
    let mut removed = VecDeque::with_capacity(instruction.quantity);
    for _ in 0..instruction.quantity {
        let taken = stacks.stacks[instruction.from - 1]
            .pop()
            .expect("stack should not be empty");
        removed.push_front(taken)
    }

    for taken in removed {
        stacks.stacks[instruction.to - 1].push(taken);
    }
}

pub fn run(input: &str) -> String {
    let [stacks, instructions] = input.splitn(2, "\n\n").collect_array();

    let mut stacks = Stacks::parse(stacks);
    for line in instructions.lines() {
        let instruction = parse_instruction(line);

        move_crates(&mut stacks, &instruction);
    }

    stacks.top()
}
