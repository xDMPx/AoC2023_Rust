pub mod aoc2023;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let day01_part1 = aoc2023::day01::part01("./test_puzzle_input/day01_part1_test.txt");
        assert_eq!(day01_part1, 142);
        let day01_part2 = aoc2023::day01::part02("./test_puzzle_input/day01_part2_test.txt");
        assert_eq!(day01_part2, 281);

        let day02_part1 = aoc2023::day02::part01("./test_puzzle_input/day02_test.txt");
        assert_eq!(day02_part1, 8);
        let day02_part2 = aoc2023::day02::part02("./test_puzzle_input/day02_test.txt");
        assert_eq!(day02_part2, 2286);

        let day03_part1 = aoc2023::day03::part01("./test_puzzle_input/day03_test.txt");
        assert_eq!(day03_part1, 4361);
        let day03_part2 = aoc2023::day03::part02("./test_puzzle_input/day03_test.txt");
        assert_eq!(day03_part2, 467835);
    }
}
