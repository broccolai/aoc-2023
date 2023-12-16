use itertools::Itertools;
use yaah::aoc;

use crate::utilities::string::parse_vec_of_numbers;

struct Input {
    histories: Vec<Vec<i32>>,
}

#[aoc(day9, part1)]
fn day9_part1(input: &'static str) -> i32 {
    parse_input(input)
        .histories
        .iter()
        .map(|history| calculate_history_differences(history))
        .map(|differences| calculate_next_value(&differences))
        .sum()
}

fn parse_input(input: &str) -> Input {
    let histories = input.lines().map(parse_vec_of_numbers).collect_vec();

    Input { histories }
}

fn calculate_history_differences(history: &[i32]) -> Vec<Vec<i32>> {
    let mut history_differences = vec![history.to_vec()];

    while history_differences
        .last()
        .is_some_and(|values| all_vec_not_zero(values))
    {
        let next = history_differences
            .last()
            .unwrap()
            .as_slice()
            .windows(2)
            .map(|window| window[1] - window[0])
            .collect_vec();

        history_differences.push(next);
    }

    history_differences
}

fn calculate_next_value(history_differences: &[Vec<i32>]) -> i32 {
    history_differences
        .iter()
        .rev()
        .skip(1)
        .fold(0i32, |last, current| last + *current.last().unwrap())
}

fn all_vec_not_zero(values: &[i32]) -> bool {
    values.iter().any(|&value| value != 0)
}

#[aoc(day9, part2)]
fn day9_part2(input: &'static str) -> i32 {
    parse_input(input)
        .histories
        .iter()
        .map(|history| calculate_history_differences(history))
        .map(|differences| calculate_previous_value(&differences))
        .sum()
}

fn calculate_previous_value(history_differences: &[Vec<i32>]) -> i32 {
    history_differences
        .iter()
        .rev()
        .skip(1)
        .fold(0i32, |last, current| *current.first().unwrap() - last)
}
