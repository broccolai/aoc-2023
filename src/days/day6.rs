use std::u64;

use crate::utilities::{parse_vec_of_numbers, remove_before_once_and_trim};
use yaah::aoc;

struct Race {
    time: u64,
    distance: u64,
}

#[aoc(day6, part1)]
fn day6_part1(input: &'static str) -> usize {
    parse_input(input).iter().map(count_ways_to_win).product()
}

fn parse_input(input: &str) -> Vec<Race> {
    let parsed_lines: Vec<Vec<u32>> = input
        .lines()
        .map(|line| remove_before_once_and_trim(line, ':'))
        .map(parse_vec_of_numbers)
        .collect();

    Iterator::zip(parsed_lines[0].iter(), parsed_lines[1].iter())
        .map(|(time, distance)| Race {
            time: u64::from(*time),
            distance: u64::from(*distance),
        })
        .collect()
}

fn count_ways_to_win(race: &Race) -> usize {
    (0..race.time)
        .filter(|&time_holding_button| {
            let travel_time = race.time - time_holding_button;
            let distance = travel_time * time_holding_button;
            distance > race.distance
        })
        .count()
}

#[aoc(day6, part2)]
fn day6_part2(input: &'static str) -> usize {
    count_ways_to_win(&parse_input_singular(input))
}

fn parse_input_singular(input: &str) -> Race {
    let parsed_lines: Vec<u64> = input
        .lines()
        .map(|line| remove_before_once_and_trim(line, ':'))
        .map(|line| line.replace(' ', ""))
        .map(|line| line.parse())
        .map(Result::unwrap)
        .collect();

    Race {
        time: parsed_lines[0],
        distance: parsed_lines[1],
    }
}
