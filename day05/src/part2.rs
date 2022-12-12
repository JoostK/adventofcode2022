use crate::common::*;
use shared::iterator::ToArray;
use std::collections::VecDeque;

pub fn run(input: &str) -> String {
    let [stacks, instructions] = input.splitn(2, "\n\n").collect_array();

    let mut stacks = Stacks::parse(stacks);
    for line in instructions.lines() {
        let instruction = Instruction::parse(line);

        let mut removed = VecDeque::with_capacity(instruction.quantity);
        for _ in 0..instruction.quantity {
            let taken = stacks.pop_from(instruction.from - 1);
            removed.push_front(taken)
        }

        stacks.stacks[instruction.to - 1].extend(removed);
    }

    stacks.top()
}
