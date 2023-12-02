pub fn part01(file_path: &str) -> usize {
    let file = std::fs::read_to_string(file_path).unwrap();
    let games_records = file.lines();

    let games_records = games_records.map(|record| {
        let mut record = record.split(':');
        let game_id = record.next().unwrap();
        let subsets = record.next().unwrap();

        let game_id = game_id
            .strip_prefix("Game ")
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let subsets = subsets.split(' ').skip(1).map(|str| str.trim());
        (game_id, subsets)
    });

    let mut id_sum = 0;
    for (game_id, mut record) in games_records {
        let mut possible = true;
        while let Some(num) = record.next() {
            let num = num.parse().unwrap();
            let color = record.next().unwrap();
            let color = color.trim_end_matches(',').trim_end_matches(';');
            possible &= is_cube_record_possible(num, color);
        }
        if possible {
            id_sum += game_id;
        }
    }

    id_sum
}

fn is_cube_record_possible(cube_num: usize, cube_color: &str) -> bool {
    match cube_color {
        "red" => cube_num <= 12,
        "green" => cube_num <= 13,
        "blue" => cube_num <= 14,
        str => unreachable!("{str}"),
    }
}

pub fn part02(file_path: &str) -> usize {
    let file = std::fs::read_to_string(file_path).unwrap();
    let games_records = file.lines();

    let games_records = games_records.map(|record| {
        let mut record = record.split(':');
        let game_id = record.next().unwrap();
        let subsets = record.next().unwrap();

        let game_id = game_id
            .strip_prefix("Game ")
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let subsets = subsets.split(' ').skip(1).map(|str| str.trim());
        (game_id, subsets)
    });

    let mut power_sum = 0;
    for (_game_id, mut record) in games_records {
        let mut min_red = usize::MIN;
        let mut min_green = usize::MIN;
        let mut min_blue = usize::MIN;

        while let Some(num) = record.next() {
            let num = num.parse().unwrap();
            let color = record.next().unwrap();
            let color = color.trim_end_matches(',').trim_end_matches(';');
            match color {
                "red" => min_red = min_red.max(num),
                "green" => min_green = min_green.max(num),
                "blue" => min_blue = min_blue.max(num),
                str => unreachable!("{str}"),
            };
        }

        power_sum += min_red * min_green * min_blue;
    }

    power_sum
}
