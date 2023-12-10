pub fn part01(file_path: &str) -> usize {
    let file = std::fs::read_to_string(file_path).unwrap();
    let sketch = file.lines();
    let map: Vec<Vec<char>> = sketch.map(|line| line.chars().collect()).collect();

    let start_pos = map
        .iter()
        .enumerate()
        .find_map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter(|(_, c)| **c == 'S')
                .map(|(x, _)| (x, y))
                .next()
        })
        .unwrap();
    let mut x = start_pos.0;
    let mut y = start_pos.1;

    let next_pos = {
        if map[x + 1][y] != '.' {
            ((x + 1), y)
        } else if map[x][y + 1] != '.' {
            (x, (y + 1))
        } else if x != 0 && map[x - 1][y] != '.' {
            ((x - 1), y)
        } else {
            (x, (y - 1))
        }
    };
    let mut x_next = next_pos.0;
    let mut y_next = next_pos.1;

    let mut steps = 0;
    while map[y_next][x_next] != 'S' {
        let pipe = map[y_next][x_next];
        for (x_diff, y_diff) in pipe_symbol_to_connections(&pipe) {
            let x_test = x_next.checked_add_signed(x_diff).unwrap();
            let y_test = y_next.checked_add_signed(y_diff).unwrap();
            if x_test != x || y_test != y {
                y = y_next;
                x = x_next;
                x_next = x_test;
                y_next = y_test;
                break;
            }
        }
        steps += 1;
    }

    (steps + 1) / 2
}

fn pipe_symbol_to_connections(pipe: &char) -> [(isize, isize); 2] {
    match pipe {
        '|' => [(0, -1), (0, 1)],
        '-' => [(1, 0), (-1, 0)],
        'L' => [(0, -1), (1, 0)],
        'J' => [(0, -1), (-1, 0)],
        '7' => [(0, 1), (-1, 0)],
        'F' => [(0, 1), (1, 0)],
        _ => unreachable!(),
    }
}
