use std::iter::Iterator;
use yaah::aoc;

const BASE_TEN: u32 = 10;

#[aoc(day1, part1)]
fn day1_part1(_input: &'static str) -> u32 {
    _input
        .lines()
        .filter_map(|line| first_and_last_digit_in_string(line))
        .map(|(first, last)| (first * 10) + last)
        .sum()
}

fn first_and_last_digit_in_string(input: &str) -> Option<(u32, u32)> {
    let first = input
        .chars()
        .find(|char| char.is_ascii_digit())
        .and_then(|char| char.to_digit(BASE_TEN));

    let last = input
        .chars()
        .rfind(|char| char.is_ascii_digit())
        .and_then(|char| char.to_digit(BASE_TEN));

    Option::zip(first, last)
}

#[aoc(day1, part2)]
fn day1_part2(_input: &'static str) -> u32 {
    _input
        .lines()
        .map(|line| first_and_last_digit_in_string_including_words(line))
        .map(|(first, last)| (first * 10) + last)
        .sum()
}

fn first_and_last_digit_in_string_including_words(input: &str) -> (u32, u32) {
    let first = search_input_for_number_directionally(input, Direction::FORWARD);
    let last = search_input_for_number_directionally(input, Direction::BACKWARD);

    (first, last)
}

#[derive(Clone, Copy)]
enum Direction {
    FORWARD,
    BACKWARD,
}

const NUMBERS: [&str; 18] = [
    "1", "one", "2", "two", "3", "three", "4", "four", "5", "five", "6", "six", "7", "seven", "8",
    "eight", "9", "nine",
];

// not perfect, had a lot of issues trying to do other things.
// leaving it at this for now but will revisit after aoc.
fn search_input_for_number_directionally(input: &str, direction: Direction) -> u32 {
    let find_directionally: fn(&str, &str) -> Option<usize> = match direction {
        Direction::FORWARD => |input, number| input.find(number),
        Direction::BACKWARD => |input, number| input.rfind(number),
    };

    let found_numbers = NUMBERS
        .iter()
        .filter_map(|&number| Option::zip(Some(number), find_directionally(input, number)));

    let (result, _) = match direction {
        Direction::FORWARD => found_numbers.min_by_key(|(_, index)| *index),
        Direction::BACKWARD => found_numbers.max_by_key(|(_, index)| *index),
    }
    .unwrap();

    let mut index = NUMBERS.iter().position(|&number| number == result).unwrap();

    if !(index % 2 == 0) {
        index = index - 1;
    }

    NUMBERS[index].parse().unwrap()
}
