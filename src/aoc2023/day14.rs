pub fn part01(file_path: &str) -> usize {
    let file = std::fs::read_to_string(file_path).unwrap();
    let map: Vec<Vec<char>> = file
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect();

    let mut tilted_map: Vec<Vec<char>> = map
        .clone()
        .iter()
        .map(|line| {
            line.iter()
                .map(|c| if *c == 'O' { '.' } else { *c })
                .collect()
        })
        .collect();
    tilted_map[0] = map[0].clone();

    for y in 1..tilted_map.len() {
        for x in 0..tilted_map[0].len() {
            if map[y][x] == 'O' {
                for y_f in (0..y).rev() {
                    if y_f == 0 && tilted_map[y_f][x] == '.' {
                        tilted_map[y_f][x] = 'O';
                        break;
                    }
                    if tilted_map[y_f][x] == 'O' || tilted_map[y_f][x] == '#' {
                        tilted_map[y_f + 1][x] = 'O';
                        break;
                    }
                }
            }
        }
    }

    tilted_map
        .iter()
        .enumerate()
        .map(|(y, line)| {
            let load = tilted_map.len() - y;
            line.iter().filter(|c| **c == 'O').count() * load
        })
        .sum()
}
