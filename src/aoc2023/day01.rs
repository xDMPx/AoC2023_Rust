pub fn part01(file_path: &str) -> usize {
    let file = std::fs::read_to_string(file_path).unwrap();
    let calibration_document = file.lines();

    let calibration_document =
        calibration_document.map(|line| line.chars().filter(|c| c.is_digit(10)));

    let mut calibration_values: Vec<usize> = Vec::new();
    calibration_document.for_each(|mut line| {
        let first_digit = line.next().unwrap();
        let last_digit = line.last().unwrap_or(first_digit);

        let calibration_value =
            first_digit.to_digit(10).unwrap() * 10 + last_digit.to_digit(10).unwrap();

        calibration_values.push(calibration_value.try_into().unwrap());
    });

    let sum = calibration_values.iter().sum();

    sum
}

pub fn part02(file_path: &str) -> usize {
    let file = std::fs::read_to_string(file_path).unwrap();

    let calibration_document = file.lines();

    let calibration_document = calibration_document.map(|line| line.chars());

    let mut calibration_values: Vec<usize> = Vec::new();
    calibration_document.for_each(|line| calibration_values.push(extract_calibration_value(line)));

    let sum = calibration_values.iter().sum();

    sum
}

fn extract_calibration_value(calibration_line: std::str::Chars<'_>) -> usize {
    let mut num_extractor = SpelledNumberExtractor::new();

    let mut first_digit = None;
    let mut last_digit = None;

    calibration_line.for_each(|c| {
        if c.is_digit(10) {
            if first_digit.is_none() {
                first_digit = Some(c.to_digit(10).unwrap().try_into().unwrap());
            } else {
                last_digit = Some(c.to_digit(10).unwrap().try_into().unwrap());
            }
        } else {
            num_extractor.add_char(c);
            let num = num_extractor.extract_num();
            if first_digit.is_none() {
                first_digit = num;
            } else if num.is_some() {
                last_digit = num;
            }
        }
    });

    let first_digit = first_digit.unwrap();
    let last_digit = last_digit.unwrap_or(first_digit);

    let calibration_value = first_digit * 10 + last_digit;

    calibration_value
}

struct SpelledNumber {
    count: usize,
    spelling: &'static str,
    num: usize,
}

struct SpelledNumberExtractor {
    spelled_number_table: [SpelledNumber; 9],
}

impl SpelledNumberExtractor {
    fn new() -> Self {
        Self {
            spelled_number_table: [
                SpelledNumber {
                    count: 0,
                    spelling: "one",
                    num: 1,
                },
                SpelledNumber {
                    count: 0,
                    spelling: "two",
                    num: 2,
                },
                SpelledNumber {
                    count: 0,
                    spelling: "three",
                    num: 3,
                },
                SpelledNumber {
                    count: 0,
                    spelling: "four",
                    num: 4,
                },
                SpelledNumber {
                    count: 0,
                    spelling: "five",
                    num: 5,
                },
                SpelledNumber {
                    count: 0,
                    spelling: "six",
                    num: 6,
                },
                SpelledNumber {
                    count: 0,
                    spelling: "seven",
                    num: 7,
                },
                SpelledNumber {
                    count: 0,
                    spelling: "eight",
                    num: 8,
                },
                SpelledNumber {
                    count: 0,
                    spelling: "nine",
                    num: 9,
                },
            ],
        }
    }

    fn add_char(&mut self, c: char) {
        for spelled_number in self.spelled_number_table.iter_mut() {
            let i = spelled_number.count;
            if let Some(spelling_char) = spelled_number.spelling.chars().skip(i).next() {
                if spelling_char == c {
                    spelled_number.count += 1;
                } else {
                    spelled_number.count = 0;
                    if spelled_number.spelling.chars().next().unwrap() == c {
                        spelled_number.count = 1;
                    }
                }
            }
        }
    }

    fn extract_num(&mut self) -> Option<usize> {
        let mut num = None;
        for spelled_number in self.spelled_number_table.iter_mut() {
            if spelled_number.count == spelled_number.spelling.len() {
                num = Some(spelled_number.num);
                spelled_number.count = 0;
                break;
            }
        }

        num
    }
}
