pub fn part01(file_path: &str) -> usize {
    let file = std::fs::read_to_string(file_path).unwrap();
    let init_sequence = file.split(',').map(|seq| hash(seq.trim()));

    init_sequence.sum()
}

fn hash(str: &str) -> usize {
    let mut current_value: usize = 0;
    str.chars()
        .filter_map(|c| c.try_into().ok())
        .filter_map(|c: u16| c.try_into().ok())
        .for_each(|code: usize| {
            current_value += code;
            current_value *= 17;
            current_value %= 256;
        });

    current_value
}
