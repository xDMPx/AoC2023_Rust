type Pattern = std::vec::Vec<std::vec::Vec<char>>;

pub fn part01(file_path: &str) -> usize {
    let file = std::fs::read_to_string(file_path).unwrap();
    let patterns: Vec<Pattern> = file
        .split("\n\n")
        .map(|p| {
            p.lines()
                .map(|line| line.trim().chars().collect())
                .collect()
        })
        .collect();

    let mut vertical = vec![];
    let mut horizontal = vec![];
    for pattern in patterns {
        if let Some(v) = find_vertical_ref(&pattern, None) {
            vertical.push(v);
        }
        if let Some(h) = find_horizontal_ref(&pattern, None) {
            horizontal.push(h);
        }
    }

    vertical.iter().sum::<usize>() + 100 * horizontal.iter().sum::<usize>()
}

pub fn part02(file_path: &str) -> usize {
    let file = std::fs::read_to_string(file_path).unwrap();
    let patterns: Vec<Pattern> = file
        .split("\n\n")
        .map(|p| {
            p.lines()
                .map(|line| line.trim().chars().collect())
                .collect()
        })
        .collect();

    let mut vertical = vec![];
    let mut horizontal = vec![];
    for pattern in patterns {
        let v = find_vertical_ref(&pattern, None).map(|v| v - 1);
        let h = find_horizontal_ref(&pattern, None).map(|h| h - 1);

        let mut reflection = false;
        let x_len = pattern[0].len();
        let y_len = pattern.len();
        for y in 0..y_len {
            for x in 0..x_len {
                let mut pattern = pattern.clone();
                pattern[y][x] = if pattern[y][x] == '.' { '#' } else { '.' };
                if let Some(v) = find_vertical_ref(&pattern, v) {
                    vertical.push(v);
                    reflection = true;
                    break;
                }
                if let Some(h) = find_horizontal_ref(&pattern, h) {
                    horizontal.push(h);
                    reflection = true;
                    break;
                }
            }
            if reflection {
                break;
            }
        }
    }

    vertical.iter().sum::<usize>() + 100 * horizontal.iter().sum::<usize>()
}

fn find_vertical_ref(pattern: &Pattern, skip: Option<usize>) -> Option<usize> {
    let x_max = pattern[0].len() - 1;
    for ref_x in (0..x_max).rev() {
        if let Some(skip) = skip {
            if skip == ref_x {
                continue;
            }
        }
        let mut reflection = false;
        for x in ref_x..x_max {
            let offset = x - ref_x;
            let r = ref_x + offset + 1;
            if offset > ref_x || r > x_max {
                break;
            }
            let l = ref_x - offset;
            reflection = pattern
                .iter()
                .map(|row| row[l] == row[r])
                .fold(true, |acc, e| acc && e);
            if !reflection {
                break;
            }
        }
        if reflection {
            return Some(ref_x + 1);
        }
    }

    None
}

fn find_horizontal_ref(pattern: &Pattern, skip: Option<usize>) -> Option<usize> {
    let y_max = pattern.len() - 1;
    for ref_y in (0..y_max).rev() {
        if let Some(skip) = skip {
            if skip == ref_y {
                continue;
            }
        }
        let mut reflection = false;
        for y in ref_y..y_max {
            let offset = y - ref_y;
            let u = ref_y + offset + 1;
            if offset > ref_y || u > y_max {
                break;
            }
            let d = ref_y - offset;
            reflection = pattern[u] == pattern[d];
            if !reflection {
                break;
            }
        }
        if reflection {
            return Some(ref_y + 1);
        }
    }

    None
}
