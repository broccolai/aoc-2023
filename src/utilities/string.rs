pub fn split_once_and_trim(source: &str, delimiter: char) -> Option<(&str, &str)> {
    source
        .split_once(delimiter)
        .map(|(first, second)| (first.trim(), second.trim()))
}

pub fn remove_before_once_and_trim(source: &str, delimiter: char) -> &str {
    source.split_once(delimiter).unwrap().1.trim()
}

pub fn parse_vec_of_numbers(source: &str) -> Vec<u32> {
    source
        .split_ascii_whitespace()
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}
