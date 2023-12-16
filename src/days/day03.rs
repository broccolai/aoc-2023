use crate::days::day03::Part::{Gear, Other};
use std::ops::Range;
use yaah::aoc;

enum Token {
    Empty,
    Part(Part),
    Number(u32),
}

enum Part {
    Gear,
    Other,
}

#[derive(Debug)]
struct SchematicNumber {
    row: usize,
    location: Range<usize>,
    value: u32,
}

#[aoc(day3, part1)]
fn day3_part1(input: &'static str) -> u32 {
    let parsed_input = parse_input(input);

    find_numbers(&parsed_input)
        .iter()
        .filter(|&schematic_number| {
            schematic_number_has_neighbouring_part(schematic_number, &parsed_input)
        })
        .map(|schematic_number| schematic_number.value)
        .sum()
}

fn parse_input(input: &str) -> Vec<Vec<Token>> {
    input.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> Vec<Token> {
    line.chars().map(parse_character).collect()
}

fn parse_character(char: char) -> Token {
    match char {
        c if c.is_ascii_digit() => Token::Number(c.to_digit(10).unwrap()),
        '.' => Token::Empty,
        '*' => Token::Part(Gear),
        _ => Token::Part(Other),
    }
}

fn find_numbers(schematic: &[Vec<Token>]) -> Vec<SchematicNumber> {
    let mut result: Vec<SchematicNumber> = Vec::new();

    for (line_index, line) in schematic.iter().enumerate() {
        let mut cursor = 0;

        while cursor < line.len() {
            let mut number = if let Token::Number(value) = line.get(cursor).unwrap() {
                *value
            } else {
                cursor += 1;
                continue;
            };

            let cursor_start = cursor;

            while let Some(next_element) = line.get(cursor + 1) {
                match next_element {
                    Token::Number(value) => {
                        cursor += 1;
                        number = (10 * number) + value;
                    }
                    _ => break,
                }
            }

            let schematic_number = SchematicNumber {
                row: line_index,
                location: cursor_start..(cursor + 1),
                value: number,
            };

            result.push(schematic_number);
            cursor += 1;
        }
    }

    result
}

fn schematic_number_has_neighbouring_part(
    schematic_number: &SchematicNumber,
    schematic: &[Vec<Token>],
) -> bool {
    for row in schematic_number.row.saturating_sub(1)..=schematic_number.row.saturating_add(1) {
        let Some(tokens) = schematic.get(row) else {
            continue;
        };

        let Range { start, end } = &schematic_number.location;

        for token_index in start.saturating_sub(1)..end.saturating_add(1) {
            match tokens.get(token_index) {
                Some(Token::Part(_)) => return true,
                _ => continue,
            }
        }
    }

    false
}

#[aoc(day3, part2)]
fn day3_part2(input: &'static str) -> u32 {
    let parsed_input = parse_input(input);
    let schematic_numbers = find_numbers(&parsed_input);

    gears_surrounded_by_two_numbers_multiplied(&schematic_numbers, &parsed_input)
}

fn gears_surrounded_by_two_numbers_multiplied(
    schematic_numbers: &[SchematicNumber],
    schematic: &[Vec<Token>],
) -> u32 {
    let mut result: Vec<u32> = Vec::new();

    for (row, line) in schematic.iter().enumerate() {
        for (column, token) in line.iter().enumerate() {
            let Token::Part(part) = token else { continue };
            if matches!(part, Other) {
                continue;
            }

            let valid_numbers: Vec<&SchematicNumber> = schematic_numbers
                .iter()
                .filter(|schematic_number| schematic_number.row.abs_diff(row) <= 1)
                .filter(|schematic_number| {
                    let mut range = column.saturating_sub(1)..=column.saturating_add(1);

                    range.any(|entry| schematic_number.location.contains(&entry))
                })
                .collect();

            if valid_numbers.len() != 2 {
                continue;
            }

            let product = valid_numbers
                .iter()
                .map(|schematic_number| schematic_number.value)
                .product();
            result.push(product);
        }
    }

    result.iter().sum()
}
