use yaah::{aoc, aoc_lib, aoc_year};

const BASE_TEN: u32 = 10;

#[aoc(day1, part1)]
fn day1_part1(_input: &'static str) -> u32 {
    _input
        .lines()
        .filter_map(|line| first_and_last_number_in_string(line))
        .map(|(first, last)| (first * 10) + last)
        .sum()
}

fn first_and_last_number_in_string(input: &str) -> Option<(u32, u32)> {
    let first = input.chars()
        .find(|char| char.is_ascii_digit())
        .and_then(|char| char.to_digit(BASE_TEN));

    let last = input.chars()
        .rfind(|char| char.is_ascii_digit())
        .and_then(|char| char.to_digit(BASE_TEN));

    Option::zip(first, last)
}