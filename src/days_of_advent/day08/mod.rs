use regex;

use crate::days_of_advent::common::io;

pub fn run() {
    let puzzle_input = io::load_input_from_file("day08");

    let puzzle_input = puzzle_input.unwrap();

    let boot_code_deserializer = BootCodeDeserializer::new();

    let program = boot_code_deserializer.deserialize(&puzzle_input);
    let mut computer = Computer {accumulated_value: 0};

    computer.run_program(&program);
    let accumulated_part_1 = computer.accumulated_value;

    let mut accumulated_part_2 = 0;

    for idx in 0..program.len() {
        let mut mut_program = program.clone();
        let original_instruction = program[idx].clone();

        let altered_instruction = match original_instruction.0 {
            InstructionCode::Jump => (InstructionCode::NoOperation, original_instruction.1),
            InstructionCode::NoOperation => (InstructionCode::Jump, original_instruction.1),
            _ => original_instruction
        };
        mut_program[idx] = altered_instruction;
        let mut computer = Computer {accumulated_value: 0};
        println!("{:?}", program);
        let termination = computer.run_program(&mut_program);
        println!("End program");
        match termination {
            TerminationMode::InfiniteLoop => (),
            TerminationMode::Halt => {accumulated_part_2 = computer.accumulated_value; break;}
        }

    }

    let content = format!(
        "\
        Accumulated value is {}\n\
        Accumulated value for part 2 is {}\n",
        accumulated_part_1,
        accumulated_part_2
    );

    let report = io::format_day_report(
        8,
        "Handheld Halting",
        "Determine value of accumulator",
        &content,
    );

    println!("{}", report);
}

struct Computer {
    pub accumulated_value: i32
}

enum TerminationMode {
    InfiniteLoop,
    Halt
}

impl Computer {
    fn run_program(&mut self, program: &[(InstructionCode, i32)]) -> TerminationMode {

        let mut visited_indices = vec![];
        let mut idx = 0;
        self.accumulated_value = 0;

        while ! visited_indices.contains(&idx) {
            if idx >= program.len() {
                return TerminationMode::Halt;
            }
            visited_indices.push(idx);
            let orig_idx = idx.clone();
            match program[idx].0 {
                InstructionCode::NoOperation => idx += 1,
                InstructionCode::Jump => idx = (idx as i32 + program[idx].1) as usize,
                InstructionCode::Accumulate => {self.accumulated_value += program[idx].1; idx += 1}
            }
        }

        return TerminationMode::InfiniteLoop;
    }
}

#[derive(Clone, Debug)]
pub enum InstructionCode {
    NoOperation,
    Accumulate,
    Jump
}

pub struct BootCodeDeserializer {
    line_regex: regex::Regex
}

impl BootCodeDeserializer {
    pub fn new() -> Self {
        BootCodeDeserializer {
            line_regex: regex::Regex::new(r"(?P<instruction>\w+) (?P<sign>\+|-)(?P<value>\d+)").unwrap()
        }
    }

    pub fn deserialize(&self, data: &str) -> Vec<(InstructionCode, i32)> {
        
        return data.trim().lines().map(|l| self.deserialize_single_line(&l)).collect::<Vec<(InstructionCode, i32)>>();
    }

    fn deserialize_single_line(&self, line: &str) -> (InstructionCode, i32) {
        let captures = self.line_regex.captures(&line).unwrap();
        let instruction = match captures.name("instruction").unwrap().as_str() {
            "nop" => InstructionCode::NoOperation,
            "acc" => InstructionCode::Accumulate,
            "jmp" => InstructionCode::Jump,
            _ => panic!("Unexpected instruction code")
        };
        let unsigned_value = captures.name("value").unwrap().as_str().parse::<i32>().unwrap();
        let value = match captures.name("sign").unwrap().as_str() {
            "-" => -unsigned_value,
            _ => unsigned_value
        };

        return (instruction, value);
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_input() {
        let input = "\
        nop +0\n\
        acc +1\n\
        jmp +4\n\
        acc +3\n\
        jmp -3\n\
        acc -99\n\
        acc +1\n\
        jmp -4\n\
        acc +6\
        ";

        let deserializer = BootCodeDeserializer::new();

        let result = deserializer.deserialize(&input);
    }

    #[test]
    fn acceptance_criteria_2() {

        let input = "\
        nop +0\n\
        acc +1\n\
        jmp +4\n\
        acc +3\n\
        jmp -3\n\
        acc -99\n\
        acc +1\n\
        jmp -4\n\
        acc +6\
        ";

        let boot_code_deserializer = BootCodeDeserializer::new();

        let program = boot_code_deserializer.deserialize(&input);

        let mut result = 0;

        for idx in 0..program.len() {
            let mut mut_program = program.clone();
            let original_instruction = program[idx].clone();

            let altered_instruction = match original_instruction.0 {
                InstructionCode::Jump => (InstructionCode::NoOperation, original_instruction.1),
                InstructionCode::NoOperation => (InstructionCode::Jump, original_instruction.1),
                _ => original_instruction
            };
            mut_program[idx] = altered_instruction;
            let mut computer = Computer {accumulated_value: 0};
            println!("{:?}", program);
            let termination = computer.run_program(&mut_program);
            println!("End program");
            match termination {
                TerminationMode::InfiniteLoop => (),
                TerminationMode::Halt => {result = computer.accumulated_value; break;}
            }

        }

        assert_eq!(8, result);
    }
}
