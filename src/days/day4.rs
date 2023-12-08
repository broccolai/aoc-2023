use std::collections::HashSet;
use yaah::aoc;
use crate::utilities::split_once_and_trim;

struct Card {
    winning_numbers: HashSet<u8>,
    owned_numbers: HashSet<u8>
}

#[aoc(day4, part1)]
fn day4_part1(input: &'static str) -> u32 {
    input.lines()
        .map(remove_input_prefix)
        .map(convert_raw_to_card)
        .map(calculate_score)
        .sum()
}

fn remove_input_prefix(input: &str) -> &str {
    split_once_and_trim(input, ':')
        .map(|(_, data)| data)
        .unwrap()
}

fn convert_raw_to_card(input: &str) -> Card {
    let (raw_winning, raw_owned) = split_once_and_trim(input, '|').unwrap();

    Card {
        winning_numbers: parse_raw_into_num_array(raw_winning),
        owned_numbers: parse_raw_into_num_array(raw_owned)
    }
}

fn parse_raw_into_num_array(input: &str) -> HashSet<u8> {
    input.split_ascii_whitespace()
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

fn calculate_score(card: Card) -> u32 {
    card.winning_numbers
        .intersection(&card.owned_numbers)
        .fold(0, |acc, _|  if acc == 0 { 1 } else { acc * 2 })
}