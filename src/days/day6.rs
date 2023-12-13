use crate::utilities::{parse_vec_of_numbers, remove_before_once_and_trim};
use yaah::aoc;

struct Race {
    time: u32,
    distance: u32,
}

#[aoc(day6, part1)]
fn day6_part1(input: &'static str) -> usize {
    parse_input(input).iter().map(count_ways_to_win).product()
}

fn parse_input(input: &'static str) -> Vec<Race> {
    let parsed_lines: Vec<Vec<u32>> = input
        .lines()
        .map(|line| remove_before_once_and_trim(line, ':'))
        .map(parse_vec_of_numbers)
        .collect();

    Iterator::zip(parsed_lines[0].iter(), parsed_lines[1].iter())
        .map(|(time, distance)| Race {
            time: *time,
            distance: *distance,
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
