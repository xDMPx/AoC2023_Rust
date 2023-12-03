struct Number {
    num: usize,
    y: usize,
    x_start: usize,
    x_end: usize,
}

struct Symbol {
    y: usize,
    x: usize,
}

pub fn part01(file_path: &str) -> usize {
    let file = std::fs::read_to_string(file_path).unwrap();
    let engine_schematic = file.lines();
    let engine_schematic = engine_schematic
        .map(|line| line.chars().enumerate())
        .enumerate();

    let mut numbers = Vec::new();
    let mut symbols = Vec::new();

    for (y, line) in engine_schematic {
        let mut num: Option<usize> = None;
        let mut x_start = 0;
        let mut x_end = 0;
        for (x, c) in line {
            if c.is_digit(10) {
                let digit = c.to_digit(10).unwrap().try_into().unwrap();
                if let Some(n) = num {
                    num = Some(10 * n + digit);
                    x_end = x;
                } else {
                    x_start = x;
                    x_end = x;
                    num = Some(digit);
                }
            } else {
                if c != '.' {
                    symbols.push(Symbol { y, x });
                }
                if let Some(num) = num {
                    numbers.push((
                        Number {
                            num,
                            y,
                            x_start,
                            x_end,
                        },
                        false,
                    ));
                }
                num = None;
            }
        }
        if let Some(num) = num {
            numbers.push((
                Number {
                    num,
                    y,
                    x_start,
                    x_end,
                },
                false,
            ));
        }
    }

    for symbol in symbols {
        let numbers = numbers.iter_mut();
        numbers.for_each(|(number, valid)| {
            *valid |= number.y <= symbol.y + 1
                && number.y >= symbol.y - 1
                && number.x_start <= symbol.x + 1
                && number.x_end >= symbol.x - 1
        });
    }

    let part_numbers = numbers.iter().filter(|(_, valid)| *valid);
    let part_numbers = part_numbers.map(|(number, _)| number.num);

    part_numbers.sum()
}

pub fn part02(file_path: &str) -> usize {
    let file = std::fs::read_to_string(file_path).unwrap();
    let engine_schematic = file.lines();
    let engine_schematic = engine_schematic
        .map(|line| line.chars().enumerate())
        .enumerate();

    let mut numbers = Vec::new();
    let mut symbols = Vec::new();

    for (y, line) in engine_schematic {
        let mut num: Option<usize> = None;
        let mut x_start = 0;
        let mut x_end = 0;
        for (x, c) in line {
            if c.is_digit(10) {
                let digit = c.to_digit(10).unwrap().try_into().unwrap();
                if let Some(n) = num {
                    num = Some(10 * n + digit);
                    x_end = x;
                } else {
                    x_start = x;
                    x_end = x;
                    num = Some(digit);
                }
            } else {
                if c == '*' {
                    symbols.push(Symbol { y, x });
                }
                if let Some(num) = num {
                    numbers.push(Number {
                        num,
                        y,
                        x_start,
                        x_end,
                    });
                }
                num = None;
            }
        }
        if let Some(num) = num {
            numbers.push(Number {
                num,
                y,
                x_start,
                x_end,
            });
        }
    }

    let mut gear_ratios_sum = 0;
    for symbol in symbols {
        let numbers = numbers.iter_mut();
        let mut gear_ratio_1 = 0;
        let mut gear_ratio_2 = 0;
        numbers.for_each(|number| {
            let valid = number.y <= symbol.y + 1
                && number.y >= symbol.y - 1
                && number.x_start <= symbol.x + 1
                && number.x_end >= symbol.x - 1;
            if valid {
                if gear_ratio_1 == 0 {
                    gear_ratio_1 = number.num;
                } else {
                    gear_ratio_2 = number.num;
                }
            }
        });
        gear_ratios_sum += gear_ratio_1 * gear_ratio_2;
    }

    gear_ratios_sum
}
