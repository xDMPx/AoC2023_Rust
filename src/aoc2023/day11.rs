pub fn part01(file_path: &str) -> usize {
    part02(file_path, 2)
}

pub fn part02(file_path: &str, expansion: usize) -> usize {
    let file = std::fs::read_to_string(file_path).unwrap();
    let image = file.lines();

    let image = image.map(|line| line.chars());

    let mut y = 0;
    let mut x_max = 0;
    let mut galaxies = Vec::new();
    for line in image {
        line.enumerate()
            .filter(|(_, c)| *c == '#')
            .for_each(|(x, _)| {
                galaxies.push((x, y));
                x_max = x_max.max(x);
            });

        y += 1;
    }

    let y_to_expand: Vec<usize> = (0..y)
        .filter(|y| galaxies.iter().filter(|(_, yg)| *yg == *y).next().is_none())
        .collect();
    let x_to_expand: Vec<usize> = (0..x_max)
        .filter(|x| galaxies.iter().filter(|(xg, _)| *xg == *x).next().is_none())
        .collect();

    galaxies.iter_mut().for_each(|(x, y)| {
        *x = *x + x_to_expand.iter().filter(|x_e| **x_e < *x).count() * (expansion - 1);
        *y = *y + y_to_expand.iter().filter(|y_e| **y_e < *y).count() * (expansion - 1);
    });

    let mut sum = 0;
    let pairs = get_pairs(1, galaxies.len());
    for pair in pairs {
        let (x_from, y_from) = galaxies[pair.0 - 1];
        let (x_to, y_to) = galaxies[pair.1 - 1];

        let dist = x_from.abs_diff(x_to) + y_from.abs_diff(y_to);
        sum += dist;
    }

    sum
}

fn get_pairs(x_max: usize, y_max: usize) -> Vec<(usize, usize)> {
    let x = x_max.min(y_max);
    let y = x_max.max(y_max);
    (x..=y)
        .map(|from| ((from + 1)..=y).map(move |to| (from, to)))
        .flatten()
        .collect()
}
