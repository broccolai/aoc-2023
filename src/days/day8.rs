use std::collections::HashMap;

use itertools::Itertools;
use yaah::aoc;

use crate::utilities::{string::split_once_and_trim, tuple::MapTuple};

struct Input {
    pattern: Vec<Instruction>,
    mappings: HashMap<String, Mapping>,
}

enum Instruction {
    Left,
    Right,
}

struct Mapping {
    left: String,
    right: String,
}

impl Mapping {
    const fn from_instruction(&self, instruction: &Instruction) -> &String {
        match instruction {
            Instruction::Left => &self.left,
            Instruction::Right => &self.right,
        }
    }
}

const STARTING_VALUE: &str = "AAA";
const TARGET_VALUE: &str = "ZZZ";

#[aoc(day8, part1)]
fn day8_part1(input: &'static str) -> usize {
    let Input { pattern, mappings } = parse_input(input);

    return pattern
        .iter()
        .cycle()
        .scan(STARTING_VALUE, |state, instruction| {
            if *state == TARGET_VALUE {
                return None;
            }

            *state = mappings.get(*state).unwrap().from_instruction(instruction);
            Some(1)
        })
        .count();
}

fn parse_input(input: &str) -> Input {
    Input {
        pattern: parse_pattern(input),
        mappings: parse_mappings(input),
    }
}

fn parse_pattern(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|char| match char {
            'L' => Instruction::Left,
            'R' => Instruction::Right,
            _ => unreachable!(),
        })
        .collect_vec()
}

fn parse_mappings(input: &str) -> HashMap<String, Mapping> {
    input
        .lines()
        .skip(2)
        .map(|line| split_once_and_trim(line, '=').unwrap())
        .map_tuple(str::to_string, parse_mapping)
        .collect()
}

fn parse_mapping(input: &str) -> Mapping {
    let trimmed_input = input.trim_start_matches('(').trim_end_matches(')');

    let (left, right) = split_once_and_trim(trimmed_input, ',').unwrap();

    Mapping {
        left: left.to_string(),
        right: right.to_string(),
    }
}
