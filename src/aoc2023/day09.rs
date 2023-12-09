pub fn part01(file_path: &str) -> usize {
    let file = std::fs::read_to_string(file_path).unwrap();
    let report = file
        .lines()
        .map(|line| line.split(' ').map(|val| val.parse::<i64>().unwrap()));

    let sum = report
        .map(|history| extrapolate(history.collect()))
        .sum::<i64>()
        .try_into()
        .unwrap();

    sum
}

fn extrapolate(hist: Vec<i64>) -> i64 {
    let mut sequence = Vec::new();

    let mut i = 0;
    while i < hist.len() - 1 {
        let diff = hist[i + 1] - hist[i];
        sequence.push(diff);
        i += 1;
    }

    let ext = {
        if sequence.iter().filter(|x| **x == 0).count() == sequence.len() {
            0
        } else {
            extrapolate(sequence)
        }
    };

    *hist.last().unwrap() + ext
}

pub fn part02(file_path: &str) -> i64 {
    let file = std::fs::read_to_string(file_path).unwrap();
    let report = file
        .lines()
        .map(|line| line.split(' ').map(|val| val.parse::<i64>().unwrap()));

    let sum = report
        .map(|history| {
            let x = extrapolate_back(history.collect());
            x
        })
        .sum();

    sum
}

fn extrapolate_back(hist: Vec<i64>) -> i64 {
    let mut sequence = Vec::new();

    let mut i = 0;
    while i < hist.len() - 1 {
        let diff = hist[i + 1] - hist[i];
        sequence.push(diff);
        i += 1;
    }

    let ext = {
        if sequence.iter().filter(|x| **x == 0).count() == sequence.len() {
            0
        } else {
            extrapolate_back(sequence)
        }
    };

    *hist.first().unwrap() - ext
}
