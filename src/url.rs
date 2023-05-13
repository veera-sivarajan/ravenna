pub fn parse_url(input: &str) -> Option<(& str, & str)> {
    input.find('/')
        .map(|index| (&input[..index], &input[index..]))
}
