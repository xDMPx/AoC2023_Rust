struct CamelCard<'a> {
    hand: &'a str,
    hand_type: usize,
    bid: usize,
}

pub fn part01(file_path: &str) -> usize {
    let file = std::fs::read_to_string(file_path).unwrap();
    let puzzle_input = file.lines();

    let camel_cards = puzzle_input.map(|line| {
        let mut input = line.split(' ');
        let hand = input.next().unwrap();
        let bid: usize = input.next().unwrap().parse().unwrap();
        CamelCard {
            hand,
            hand_type: get_hand_type(hand),
            bid,
        }
    });

    let mut camel_cards: Vec<CamelCard> = camel_cards.collect();
    camel_cards.sort_unstable_by(|a, b| a.simple_cmp(b));
    let total_winnings = camel_cards
        .iter()
        .enumerate()
        .fold(0, |acc, (i, x_camel_card)| acc + (i + 1) * x_camel_card.bid);

    total_winnings
}

impl CamelCard<'_> {
    fn simple_cmp(&self, other: &CamelCard) -> std::cmp::Ordering {
        let type_cmp = self.hand_type.cmp(&other.hand_type);

        if type_cmp == std::cmp::Ordering::Equal {
            for (a, b) in self.hand.chars().zip(other.hand.chars()) {
                if a != b {
                    return get_card_strength(&a).cmp(&get_card_strength(&b));
                }
            }
            std::cmp::Ordering::Equal
        } else {
            type_cmp
        }
    }
}

fn get_card_strength(card: &char) -> usize {
    match card {
        '2'..='9' => (card.to_digit(10).unwrap() as usize) - 2,
        'T' => 8,
        'J' => 9,
        'Q' => 10,
        'K' => 11,
        'A' => 12,
        _ => unreachable!(),
    }
}

fn get_hand_type(hand: &str) -> usize {
    let mut strengths_count = [0; 13];

    hand.chars()
        .for_each(|card| strengths_count[get_card_strength(&card)] += 1);

    let max_strength_count = strengths_count.iter().max().unwrap();
    match max_strength_count {
        1 => 0,
        2 => strengths_count.iter().filter(|x| **x == 2).count(),
        3 => 3 + strengths_count.iter().filter(|x| **x == 2).count(),
        4 => 5,
        5 => 6,
        _ => unreachable!(),
    }
}

pub fn part02(file_path: &str) -> usize {
    let file = std::fs::read_to_string(file_path).unwrap();
    let puzzle_input = file.lines();

    let camel_cards = puzzle_input.map(|line| {
        let mut input = line.split(' ');
        let hand = input.next().unwrap();
        let bid: usize = input.next().unwrap().parse().unwrap();
        CamelCard {
            hand,
            hand_type: get_hand_type_2(hand),
            bid,
        }
    });

    let mut camel_cards: Vec<CamelCard> = camel_cards.collect();
    camel_cards.sort_unstable_by(|a, b| a.simple_cmp_2(b));
    let total_winnings = camel_cards
        .iter()
        .enumerate()
        .fold(0, |acc, (i, x_camel_card)| acc + (i + 1) * x_camel_card.bid);

    total_winnings
}

impl CamelCard<'_> {
    fn simple_cmp_2(&self, other: &CamelCard) -> std::cmp::Ordering {
        let type_cmp = self.hand_type.cmp(&other.hand_type);

        if type_cmp == std::cmp::Ordering::Equal {
            for (a, b) in self.hand.chars().zip(other.hand.chars()) {
                if a != b {
                    return get_card_strength_2(&a).cmp(&get_card_strength_2(&b));
                }
            }
            std::cmp::Ordering::Equal
        } else {
            type_cmp
        }
    }
}

fn get_card_strength_2(card: &char) -> usize {
    match card {
        'J' => 0,
        '2'..='9' => (card.to_digit(10).unwrap() as usize) - 1,
        'T' => 9,
        'Q' => 10,
        'K' => 11,
        'A' => 12,
        _ => unreachable!(),
    }
}

fn get_hand_type_2(hand: &str) -> usize {
    let mut strengths_count = [0; 13];

    hand.chars()
        .for_each(|card| strengths_count[get_card_strength_2(&card)] += 1);

    let jokers_count = strengths_count[0];
    let strengths_count = &mut strengths_count[1..];
    let (max_i, max_strength_count) = strengths_count
        .iter()
        .enumerate()
        .max_by_key(|(_, strength_count)| *strength_count)
        .map(|(i, strength_count)| (i, *strength_count))
        .unwrap();
    strengths_count[max_i] += jokers_count;
    match max_strength_count + jokers_count {
        1 => 0,
        2 => strengths_count.iter().filter(|x| **x == 2).count(),
        3 => 3 + strengths_count.iter().filter(|x| **x == 2).count(),
        4 => 5,
        5 => 6,
        _ => unreachable!(),
    }
}
