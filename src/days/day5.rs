use yaah::aoc;

const _SAMPLE: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u32>,
    conversions: Vec<Conversion>,
}

#[derive(Debug)]
struct Conversion {
    steps: Vec<ConversionStep>,
}

#[derive(Debug)]
struct ConversionStep {
    destination_start: u32,
    source_start: u32,
    length: u32,
}

#[aoc(day5, part1)]
fn day5_part1(input: &'static str) -> u32 {
    let almanac = parse_input(input);

    *process_almanac(&almanac).iter().min().unwrap()
}

fn parse_input(input: &'static str) -> Almanac {
    let seeds: Vec<u32> = input
        .lines()
        .next()
        .unwrap()
        .trim_start_matches("seeds:")
        .split_ascii_whitespace()
        .map(str::parse::<u32>)
        .map(Result::unwrap)
        .collect();

    let mut conversions: Vec<Conversion> = Vec::new();
    let mut steps: Vec<ConversionStep> = Vec::new();

    for line in input.lines().skip(2) {
        if line.is_empty() {
            conversions.push(Conversion { steps });
            steps = Vec::new();
            continue;
        }

        if line.contains(':') {
            continue;
        }

        let parsed_numbers: Vec<u32> = line
            .split_ascii_whitespace()
            .map(str::parse::<u32>)
            .map(Result::unwrap)
            .collect();

        steps.push(ConversionStep {
            destination_start: parsed_numbers[0],
            source_start: parsed_numbers[1],
            length: parsed_numbers[2],
        });
    }

    if !steps.is_empty() {
        conversions.push(Conversion { steps });
    }

    Almanac { seeds, conversions }
}

fn process_almanac(almanac: &Almanac) -> Vec<u32> {
    let Almanac { seeds, conversions } = almanac;
    let mut processed_seeds: Vec<u32> = Vec::new();

    for seed in seeds {
        let mut processed_seed = *seed;

        for conversion in conversions {
            for step in &conversion.steps {
                if let Some(value) = apply_step_to_seed(step, &processed_seed) {
                    processed_seed = value;
                    break;
                }
            }
        }

        processed_seeds.push(processed_seed);
    }

    processed_seeds
}

fn apply_step_to_seed(step: &ConversionStep, seed: &u32) -> Option<u32> {
    if &step.source_start > seed {
        return None;
    }

    let adjusted = seed - step.source_start;

    if adjusted > step.length {
        return None;
    }

    Some(step.destination_start + adjusted)
}
