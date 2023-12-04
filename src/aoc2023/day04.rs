use std::collections::VecDeque;

pub fn part01(file_path: &str) -> usize {
    let file = std::fs::read_to_string(file_path).unwrap();
    let scratchcards = file.lines().map(|line| {
        let card = (&line[line.find(':').unwrap() + 1..]).trim();

        let mut card = card.split('|');
        let winning_numbers = card.next().unwrap().trim();
        let numbers = card.next().unwrap().trim();

        let winning_numbers = winning_numbers
            .split(' ')
            .filter_map(|num| num.parse::<usize>().ok());
        let numbers = numbers
            .split(' ')
            .filter_map(|num| num.parse::<usize>().ok());

        (winning_numbers, numbers)
    });

    let mut points = 0;
    for (winning_numbers, numbers) in scratchcards {
        let winning_numbers: Vec<usize> = winning_numbers.collect();
        let matches = numbers.filter(|num| winning_numbers.contains(num));
        let matches_count: u32 = matches.count().try_into().unwrap();
        if matches_count > 0 {
            points += 1 * usize::pow(2, matches_count - 1);
        }
    }

    points
}

pub fn part02(file_path: &str) -> usize {
    let file = std::fs::read_to_string(file_path).unwrap();
    let scratchcards = file.lines().map(|line| {
        let card = (&line[line.find(':').unwrap() + 1..]).trim();

        let mut card = card.split('|');
        let winning_numbers = card.next().unwrap().trim();
        let numbers = card.next().unwrap().trim();

        let winning_numbers = winning_numbers
            .split(' ')
            .filter_map(|num| num.parse::<usize>().ok());
        let numbers = numbers
            .split(' ')
            .filter_map(|num| num.parse::<usize>().ok());

        (winning_numbers, numbers)
    });

    let mut total_scratchcards = 0;
    let mut copies_count = VecDeque::<usize>::new();
    for (winning_numbers, numbers) in scratchcards {
        let instances = 1 + copies_count.pop_front().unwrap_or(0);
        let winning_numbers: Vec<usize> = winning_numbers.collect();
        let matches = numbers.filter(|num| winning_numbers.contains(num));
        let matches_count = matches.count();
        for i in 0..matches_count {
            if let Some(count) = copies_count.get_mut(i) {
                *count += instances;
            } else {
                copies_count.push_back(instances);
            }
        }

        total_scratchcards += instances;
    }

    total_scratchcards
}
