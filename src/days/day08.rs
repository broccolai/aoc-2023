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
    find_steps_for_value(&parse_input(input), STARTING_VALUE, |value| {
        value == TARGET_VALUE
    })
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

fn find_steps_for_value(input: &Input, value: &str, target: fn(&str) -> bool) -> usize {
    let Input { pattern, mappings } = input;

    return pattern
        .iter()
        .cycle()
        .scan(value, |state, instruction| {
            if target(state) {
                return None;
            }

            *state = mappings.get(*state).unwrap().from_instruction(instruction);
            Some(1)
        })
        .count();
}

const END: &str = "Z";

#[aoc(day8, part2)]
fn day8_part2(input: &'static str) -> i64 {
    let parsed_input = parse_input(input);

    parsed_input
        .mappings
        .keys()
        .filter(|key| key.ends_with("A"))
        .map(|value| find_steps_for_value(&parsed_input, value, |current| current.ends_with(END)))
        .map(|number| number as i64)
        .fold(1i64, lcm)
}

const fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a.abs()
}

const fn lcm(a: i64, b: i64) -> i64 {
    a * b / gcd(a, b)
}
