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
    }
}
