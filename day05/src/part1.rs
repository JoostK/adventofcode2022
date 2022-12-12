use crate::common::*;
use shared::iterator::ToArray;

pub fn run(input: &str) -> String {
    let [stacks, instructions] = input.splitn(2, "\n\n").collect_array();

    let mut stacks = Stacks::parse(stacks);
    for line in instructions.lines() {
        let instruction = Instruction::parse(line);

        for _ in 0..instruction.quantity {
            let taken = stacks.pop_from(instruction.from - 1);
            stacks.stacks[instruction.to - 1].push(taken);
        }
    }

    stacks.top()
}
