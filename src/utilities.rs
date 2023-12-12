pub fn split_once_and_trim(source: &str, delimiter: char) -> Option<(&str, &str)> {
    source
        .split_once(delimiter)
        .map(|(first, second)| (first.trim(), second.trim()))
}
