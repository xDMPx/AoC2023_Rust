pub fn part01(file_path: &str) -> usize {
    let file = std::fs::read_to_string(file_path).unwrap();
    let init_sequence = file.split(',').map(|seq| hash(seq.trim()));

    init_sequence.sum()
}

pub fn part02(file_path: &str) -> usize {
    let file = std::fs::read_to_string(file_path).unwrap();
    let init_sequence = file.split(',').map(|seq| {
        let seq = seq.trim();
        if seq.ends_with('-') {
            let seq = &seq[..seq.len() - 1];
            Operation::DASH((hash(&seq), seq.to_owned()))
        } else {
            let seq = seq.split_once('=').unwrap();
            let hash = hash(seq.0);
            Operation::EQUAL((
                hash,
                Label {
                    label: seq.0.to_owned(),
                    focal_length: seq.1.to_owned().parse().unwrap(),
                },
            ))
        }
    });

    const ARRAY_REPEAT_VALUE: Vec<Label> = Vec::new();
    let mut boxes: [Vec<Label>; 256] = [ARRAY_REPEAT_VALUE; 256];

    for operation in init_sequence {
        match operation {
            Operation::DASH((box_num, label)) => {
                if let Some((i, _)) = boxes[box_num]
                    .iter()
                    .enumerate()
                    .find(|(_, l)| l.label == label)
                {
                    boxes[box_num].remove(i);
                }
            }
            Operation::EQUAL((box_num, label)) => {
                if let Some((i, _)) = boxes[box_num]
                    .iter()
                    .enumerate()
                    .find(|(_, l)| l.label == label.label)
                {
                    boxes[box_num][i] = label;
                } else {
                    boxes[box_num].push(label);
                }
            }
        }
    }

    boxes
        .iter()
        .enumerate()
        .map(|(i, b)| {
            b.iter()
                .enumerate()
                .map(|(j, lens)| (i + 1) * (j + 1) * lens.focal_length)
                .sum::<usize>()
        })
        .sum()
}

enum Operation {
    DASH((usize, String)),
    EQUAL((usize, Label)),
}

struct Label {
    label: String,
    focal_length: usize,
}

fn hash(str: &str) -> usize {
    let mut current_value: usize = 0;
    str.chars()
        .filter_map(|c| c.try_into().ok())
        .filter_map(|c: u16| c.try_into().ok())
        .for_each(|code: usize| {
            current_value += code;
            current_value *= 17;
            current_value %= 256;
        });

    current_value
}
