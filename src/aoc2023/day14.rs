pub fn part01(file_path: &str) -> usize {
    let file = std::fs::read_to_string(file_path).unwrap();
    let map: Vec<Vec<char>> = file
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect();

    let tilted_map = tilt_map_north(&map);

    tilted_map
        .iter()
        .enumerate()
        .map(|(y, line)| {
            let load = tilted_map.len() - y;
            line.iter().filter(|c| **c == 'O').count() * load
        })
        .sum()
}

pub fn part02(file_path: &str) -> usize {
    let file = std::fs::read_to_string(file_path).unwrap();
    let map: Vec<Vec<char>> = file
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect();

    let mut tilted_map = map;
    let mut completed_cycles = 0;
    for _ in 0..4 * tilted_map.len() {
        tilted_map = tilt_map_north(&tilted_map);
        tilted_map = tilt_map_west(&tilted_map);
        tilted_map = tilt_map_south(&tilted_map);
        tilted_map = tilt_map_east(&tilted_map);

        completed_cycles += 1;
    }

    let mut cycles_search_state = vec![];
    let mut cycles: Vec<[(Vec<char>, usize, usize); 2]> = vec![];
    tilted_map.iter().for_each(|_| cycles_search_state.push(0));
    tilted_map
        .iter()
        .for_each(|_| cycles.push([(vec![], 0, 0), (vec![], 0, 0)]));

    for _ in 0..4 * tilted_map.len() {
        let cycle_start_map = tilted_map.clone();
        tilted_map = tilt_map_north(&tilted_map);
        tilted_map = tilt_map_west(&tilted_map);
        tilted_map = tilt_map_south(&tilted_map);
        tilted_map = tilt_map_east(&tilted_map);
        completed_cycles += 1;

        for i in 0..tilted_map.len() {
            match cycles_search_state[i] {
                0 => {
                    if tilted_map[i] != cycle_start_map[i] {
                        cycles[i][0] = (tilted_map[i].clone(), 1, completed_cycles);
                        cycles_search_state[i] = 1;
                    }
                }
                1 => {
                    if tilted_map[i] == cycle_start_map[i] {
                        cycles[i][0].1 += 1;
                    } else {
                        cycles[i][1] = (tilted_map[i].clone(), 1, completed_cycles);
                        cycles_search_state[i] = 2;
                    }
                }
                2 => {
                    if tilted_map[i] == cycle_start_map[i] {
                        cycles[i][1].1 += 1;
                    } else {
                        cycles_search_state[i] = 3;
                    }
                }
                3 => continue,
                _ => unreachable!(),
            }
        }
    }

    let target = 1000000000;
    let map: Vec<Vec<char>> = cycles
        .iter()
        .enumerate()
        .map(|(y, cycle)| {
            if cycle[0].0.len() == 0 {
                tilted_map[y].clone()
            } else {
                if (target - cycle[0].2) % (cycle[0].1 + cycle[1].1) < cycle[0].1 {
                    cycle[0].0.clone()
                } else {
                    cycle[1].0.clone()
                }
            }
        })
        .collect();

    map.iter()
        .enumerate()
        .map(|(y, line)| {
            let load = tilted_map.len() - y;
            line.iter().filter(|c| **c == 'O').count() * load
        })
        .sum()
}

fn tilt_map_north(map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
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
}

fn tilt_map_south(map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let max_y = map.len() - 1;
    let mut tilted_map: Vec<Vec<char>> = map
        .clone()
        .iter()
        .map(|line| {
            line.iter()
                .map(|c| if *c == 'O' { '.' } else { *c })
                .collect()
        })
        .collect();
    tilted_map[max_y] = map[max_y].clone();

    for y in (0..tilted_map.len() - 1).rev() {
        for x in 0..tilted_map[0].len() {
            if map[y][x] == 'O' {
                for y_f in y..=max_y {
                    if y_f == max_y && tilted_map[y_f][x] == '.' {
                        tilted_map[y_f][x] = 'O';
                        break;
                    }
                    if tilted_map[y_f][x] == 'O' || tilted_map[y_f][x] == '#' {
                        tilted_map[y_f - 1][x] = 'O';
                        break;
                    }
                }
            }
        }
    }

    tilted_map
}

fn tilt_map_west(map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut tilted_map: Vec<Vec<char>> = map
        .clone()
        .iter()
        .map(|line| {
            line.iter()
                .enumerate()
                .map(|(x, c)| if *c == 'O' && x != 0 { '.' } else { *c })
                .collect()
        })
        .collect();

    for y in 0..tilted_map.len() {
        for x in 1..tilted_map[0].len() {
            if map[y][x] == 'O' {
                for x_f in (0..x).rev() {
                    if x_f == 0 && tilted_map[y][x_f] == '.' {
                        tilted_map[y][x_f] = 'O';
                        break;
                    }
                    if tilted_map[y][x_f] == 'O' || tilted_map[y][x_f] == '#' {
                        tilted_map[y][x_f + 1] = 'O';
                        break;
                    }
                }
            }
        }
    }

    tilted_map
}

fn tilt_map_east(map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let max_x = map[0].len() - 1;
    let mut tilted_map: Vec<Vec<char>> = map
        .clone()
        .iter()
        .map(|line| {
            line.iter()
                .enumerate()
                .map(|(x, c)| if *c == 'O' && x != max_x { '.' } else { *c })
                .collect()
        })
        .collect();

    for y in 0..tilted_map.len() {
        for x in (0..tilted_map[0].len() - 1).rev() {
            if map[y][x] == 'O' {
                for x_f in x..=max_x {
                    if x_f == max_x && tilted_map[y][x_f] == '.' {
                        tilted_map[y][x_f] = 'O';
                        break;
                    }
                    if tilted_map[y][x_f] == 'O' || tilted_map[y][x_f] == '#' {
                        tilted_map[y][x_f - 1] = 'O';
                        break;
                    }
                }
            }
        }
    }

    tilted_map
}
