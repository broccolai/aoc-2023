use std::iter::Iterator;
use yaah::aoc;

const BASE_TEN: u32 = 10;

#[aoc(day1, part1)]
fn day1_part1(input: &'static str) -> u32 {
    input
        .lines()
        .filter_map(first_and_last_digit_in_string)
        .map(|(first, last)| (first * 10) + last)
        .sum()
}

fn first_and_last_digit_in_string(input: &str) -> Option<(u32, u32)> {
    let first = input
        .chars()
        .find(char::is_ascii_digit)
        .and_then(|char| char.to_digit(BASE_TEN));

    let last = input
        .chars()
        .rfind(char::is_ascii_digit)
        .and_then(|char| char.to_digit(BASE_TEN));

    Option::zip(first, last)
}

#[aoc(day1, part2)]
fn day1_part2(input: &'static str) -> u32 {
    input
        .lines()
        .map(first_and_last_digit_in_string_including_words)
        .map(|(first, last)| (first * 10) + last)
        .sum()
}

fn first_and_last_digit_in_string_including_words(input: &str) -> (u32, u32) {
    let first = search_input_for_number_directionally(input, Direction::Forward);
    let last = search_input_for_number_directionally(input, Direction::Backward);

    (first, last)
}

#[derive(Clone, Copy)]
enum Direction {
    Forward,
    Backward,
}

const NUMBERS: [&str; 18] = [
    "1", "one", "2", "two", "3", "three", "4", "four", "5", "five", "6", "six", "7", "seven", "8",
    "eight", "9", "nine",
];

// not perfect, had a lot of issues trying to do other things.
// leaving it at this for now but will revisit after aoc.
fn search_input_for_number_directionally(input: &str, direction: Direction) -> u32 {
    let find_directionally: fn(&str, &str) -> Option<usize> = match direction {
        Direction::Forward => |input, number| input.find(number),
        Direction::Backward => |input, number| input.rfind(number),
    };

    let found_numbers = NUMBERS
        .iter()
        .filter_map(|&number| Option::zip(Some(number), find_directionally(input, number)));

    let (result, _) = match direction {
        Direction::Forward => found_numbers.min_by_key(|(_, index)| *index),
        Direction::Backward => found_numbers.max_by_key(|(_, index)| *index),
    }
    .unwrap();

    let mut index = NUMBERS.iter().position(|&number| number == result).unwrap();

    if index % 2 != 0 {
        index -= 1;
    }

    NUMBERS[index].parse().unwrap()
}
