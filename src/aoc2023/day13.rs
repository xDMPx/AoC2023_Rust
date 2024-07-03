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
        let x_max = pattern[0].len() - 1;
        for ref_x in (0..x_max).rev() {
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
                vertical.push(ref_x + 1);
                break;
            }
        }

        let y_max = pattern.len() - 1;
        for ref_y in (0..y_max).rev() {
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
                horizontal.push(ref_y + 1);
                break;
            }
        }
    }

    vertical.iter().sum::<usize>() + 100 * horizontal.iter().sum::<usize>()
}
