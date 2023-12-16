use std::cmp::Ordering;

use crate::utilities::tuple::MapTuple;
use itertools::Itertools;
use yaah::aoc;

struct Hand {
    cards: Vec<char>,
    bid: usize,
}

#[aoc(day7, part1)]
fn day7_part1(input: &'static str) -> usize {
    parse_input(input, false)
        .iter()
        .sorted_by(|source, target| ordering_by_hand_value(source, target))
        .rev()
        .enumerate()
        .map(|(index, hand)| hand.bid * (index + 1))
        .sum()
}

fn parse_input(input: &str, replace_jokers: bool) -> Vec<Hand> {
    let replaced_lines = input
        .lines()
        .map(|str| {
            if replace_jokers {
                str.replace('J', "X")
            } else {
                str.to_string()
            }
        })
        .collect_vec();

    replaced_lines
        .into_iter()
        .filter_map(|line| {
            line.split_once(' ')
                .map(|(a, b)| (a.to_string(), b.to_string()))
        })
        .map_tuple(
            |cards| cards.chars().collect_vec(),
            |num| num.parse().unwrap(),
        )
        .map(|(cards, bid)| Hand { cards, bid })
        .collect()
}

fn ordering_by_hand_value(source_hand: &Hand, target_hand: &Hand) -> Ordering {
    let mut comparitor = Ord::cmp(
        &calculate_card_suite_value(source_hand),
        &calculate_card_suite_value(target_hand),
    );

    for (self_card, other_card) in Iterator::zip(source_hand.cards.iter(), target_hand.cards.iter())
    {
        comparitor = comparitor.then_with(|| ordering_card_by_value(*self_card, *other_card));
    }

    comparitor
}

fn ordering_card_by_value(source: char, target: char) -> Ordering {
    Ord::cmp(&card_value(target), &card_value(source))
}

// hated this task to be honest so didn't bother structuring it in a nice way
fn calculate_card_suite_value(hand: &Hand) -> u32 {
    let mut counts_by_card = hand
        .cards
        .iter()
        .into_group_map_by(|&card| card)
        .iter()
        .map_tuple(|&card| card, Vec::len)
        .sorted_by(
            |&(source_card, source_count), &(target_card, target_count)| {
                Ord::cmp(&source_count, &target_count)
                    .then_with(|| ordering_card_by_value(*source_card, *target_card))
            },
        )
        .rev()
        .collect_vec();

    let amount_of_jokers = counts_by_card
        .iter()
        .position(|(&card, _count)| card == 'X')
        .map(|index| counts_by_card.remove(index));

    if let Some(result) = amount_of_jokers {
        if counts_by_card.is_empty() {
            counts_by_card.push(result);
        } else {
            let (target_card, target_amount) = counts_by_card[0];
            counts_by_card[0] = (target_card, target_amount + result.1);
        };
    };

    let counts = counts_by_card
        .iter()
        .map(|(_card, count)| count)
        .collect_vec();

    match counts.as_slice() {
        [5, ..] => 1,
        [4, ..] => 2,
        [3, 2] => 3,
        [3, ..] => 4,
        [2, 2, ..] => 5,
        [2, ..] => 6,
        _ => 7,
    }
}

const fn card_value(card: char) -> u32 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        'X' => 1,
        _ => 0,
    }
}

#[aoc(day7, part2)]
fn day7_part2(input: &'static str) -> usize {
    parse_input(input, true)
        .iter()
        .sorted_by(|source, target| ordering_by_hand_value(source, target))
        .rev()
        .enumerate()
        .map(|(index, hand)| hand.bid * (index + 1))
        .sum()
}
