pub fn part01(file_path: &str) -> usize {
    let file = std::fs::read_to_string(file_path).unwrap();
    let mut document = file.lines().map(|line| {
        let (_, numbers) = line.split_once(':').unwrap();
        numbers
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<usize>().unwrap())
    });

    let times = document.next().unwrap();
    let distances = document.next().unwrap();

    let mut number_of_ways = 1;
    for (time, distance) in times.zip(distances) {
        let w_hold_times = (1..time)
            .map(|i| {
                let t = time - i;
                let v = i;
                let d = v * t;
                d
            })
            .filter(|d| *d > distance);

        number_of_ways *= w_hold_times.count()
    }

    number_of_ways
}

pub fn part02(file_path: &str) -> usize {
    let file = std::fs::read_to_string(file_path).unwrap();
    let mut document = file.lines().map(|line| -> usize {
        let (_, numbers) = line.split_once(':').unwrap();
        numbers
            .chars()
            .filter(|c| c.is_digit(10))
            .map(|c| c.to_digit(10).unwrap().try_into().unwrap())
            .reduce(|acc, e| acc * 10 + e)
            .unwrap()
    });

    let time = document.next().unwrap();
    let distance = document.next().unwrap();

    let winning_time_start = (1..time)
        .map(|i| {
            let t = time - i;
            let v = i;
            let d = v * t;
            (i, d)
        })
        .filter(|(_, d)| *d > distance)
        .next()
        .unwrap()
        .0;

    time - 2 * winning_time_start + 1
}
