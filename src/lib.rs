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

        let day04_part1 = aoc2023::day04::part01("./test_puzzle_input/day04_test.txt");
        assert_eq!(day04_part1, 13);
        let day04_part2 = aoc2023::day04::part02("./test_puzzle_input/day04_test.txt");
        assert_eq!(day04_part2, 30);

        let day05_part1 = aoc2023::day05::part01("./test_puzzle_input/day05_test.txt");
        assert_eq!(day05_part1, 35);
        let day05_part2 = aoc2023::day05::part02("./test_puzzle_input/day05_test.txt");
        assert_eq!(day05_part2, 46);

        let day06_part1 = aoc2023::day06::part01("./test_puzzle_input/day06_test.txt");
        assert_eq!(day06_part1, 288);
        let day06_part2 = aoc2023::day06::part02("./test_puzzle_input/day06_test.txt");
        assert_eq!(day06_part2, 71503);
    }
}
