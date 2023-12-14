use crate::utilities::string::split_once_and_trim;
use std::collections::HashSet;
use yaah::aoc;

struct Card {
    winning_numbers: HashSet<u8>,
    owned_numbers: HashSet<u8>,
}

#[aoc(day4, part1)]
fn day4_part1(input: &'static str) -> usize {
    input
        .lines()
        .map(parse_into_card)
        .map(|card| calculate_score(&card))
        .sum()
}

fn parse_into_card(input: &str) -> Card {
    let (_, raw_numbers) = split_once_and_trim(input, ':').unwrap();
    let (raw_winning, raw_owned) = split_once_and_trim(raw_numbers, '|').unwrap();

    Card {
        winning_numbers: parse_into_num_array(raw_winning),
        owned_numbers: parse_into_num_array(raw_owned),
    }
}

fn parse_into_num_array(input: &str) -> HashSet<u8> {
    input
        .split_ascii_whitespace()
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

fn calculate_score(card: &Card) -> usize {
    card.winning_numbers
        .intersection(&card.owned_numbers)
        .fold(0, |acc, _| if acc == 0 { 1 } else { acc * 2 })
}

#[aoc(day4, part2)]
fn day4_part2(input: &'static str) -> u32 {
    let cards: Vec<Card> = input.lines().map(parse_into_card).collect();

    calculate_amount_of_cards(&cards)
}

fn calculate_amount_of_cards(cards: &[Card]) -> u32 {
    let mut card_amounts: Vec<u32> = vec![1; cards.len()];

    for (card_index, card) in cards.iter().enumerate() {
        let amount_of_card = card_amounts[card_index];

        let score = winning_intersection_size(card);
        let next_index = card_index + 1;

        for index in next_index..next_index + score {
            if let Some(amount) = card_amounts.get_mut(index) {
                *amount += amount_of_card;
            };
        }
    }

    card_amounts.iter().sum()
}

fn winning_intersection_size(card: &Card) -> usize {
    card.winning_numbers
        .intersection(&card.owned_numbers)
        .count()
}
