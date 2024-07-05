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

        let day07_part1 = aoc2023::day07::part01("./test_puzzle_input/day07_test.txt");
        assert_eq!(day07_part1, 6440);
        let day07_part2 = aoc2023::day07::part02("./test_puzzle_input/day07_test.txt");
        assert_eq!(day07_part2, 5905);

        let day08_part1 = aoc2023::day08::part01("./test_puzzle_input/day08_test.txt");
        assert_eq!(day08_part1, 2);
        let day08_part1_2 = aoc2023::day08::part01("./test_puzzle_input/day08_test_2.txt");
        assert_eq!(day08_part1_2, 6);
        let day08_part2 = aoc2023::day08::part02("./test_puzzle_input/day08_part2_test.txt");
        assert_eq!(day08_part2, 6);

        let day09_part1 = aoc2023::day09::part01("./test_puzzle_input/day09_test.txt");
        assert_eq!(day09_part1, 114);
        let day09_part2 = aoc2023::day09::part02("./test_puzzle_input/day09_test.txt");
        assert_eq!(day09_part2, 2);

        let day10_part1 = aoc2023::day10::part01("./test_puzzle_input/day10_test.txt");
        assert_eq!(day10_part1, 8);
        /*
        let day10_part2 = aoc2023::day10::part02("./test_puzzle_input/day10_test.txt");
        assert_eq!(day10_part2, 2);
        */

        let day11_part1 = aoc2023::day11::part01("./test_puzzle_input/day11_test.txt");
        assert_eq!(day11_part1, 374);
        let day11_part2 = aoc2023::day11::part02("./test_puzzle_input/day11_test.txt", 10);
        assert_eq!(day11_part2, 1030);
        let day11_part2 = aoc2023::day11::part02("./test_puzzle_input/day11_test.txt", 100);
        assert_eq!(day11_part2, 8410);

        let day13_part1 = aoc2023::day13::part01("./test_puzzle_input/day13_test.txt");
        assert_eq!(day13_part1, 405);
        let day13_part2 = aoc2023::day13::part02("./test_puzzle_input/day13_test.txt");
        assert_eq!(day13_part2, 400);

        let day14_part1 = aoc2023::day14::part01("./test_puzzle_input/day14_test.txt");
        assert_eq!(day14_part1, 136);
        //let day14_part2 = aoc2023::day14::part02("./test_puzzle_input/day14_test.txt");
        //assert_eq!(day14_part2, 64);

        let day15_part1 = aoc2023::day15::part01("./test_puzzle_input/day15_test.txt");
        assert_eq!(day15_part1, 1320);
    }
}
