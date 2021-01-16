use regex;

use crate::days_of_advent::common::io;

pub fn run() {
    let puzzle_input = io::load_input_from_file("day08");

    let puzzle_input = puzzle_input.unwrap();

    let boot_code_deserializer = BootCodeDeserializer::new();

    let program = boot_code_deserializer.deserialize(&puzzle_input);

    let mut visited_indices = vec![];
    let mut idx = 0;
    let mut accumulated_value = 0;

    while ! visited_indices.contains(&idx) {
        visited_indices.push(idx);
        match program[idx].0 {
            InstructionCode::NoOperation => idx += 1,
            InstructionCode::Jump => idx = (idx as i32 + program[idx].1) as usize,
            InstructionCode::Accumulate => {accumulated_value += program[idx].1; idx += 1}
        }

    }

    let content = format!(
        "\
        Accumulated value is {}",
        accumulated_value
    );

    let report = io::format_day_report(
        8,
        "Handheld Halting",
        "Determine value of accumulator",
        &content,
    );

    println!("{}", report);
}

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
}
