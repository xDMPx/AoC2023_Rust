struct Mapper {
    destination_rs: usize,
    source_rs: usize,
    range_length: usize,
}

impl Mapper {
    fn in_source_range(&self, num: usize) -> bool {
        num >= self.source_rs && num < self.source_rs + self.range_length
    }

    fn map(&self, num: usize) -> Option<usize> {
        if self.in_source_range(num) {
            Some(num - self.source_rs + self.destination_rs)
        } else {
            None
        }
    }

    fn get_source_end(&self) -> usize {
        self.source_rs + self.range_length
    }
}

pub fn part01(file_path: &str) -> usize {
    let file = std::fs::read_to_string(file_path).unwrap();
    let mut almanac = file.lines().filter(|line| !line.is_empty());

    let seeds = almanac
        .next()
        .unwrap()
        .trim_start_matches("seeds: ")
        .split(' ')
        .map(|s| s.parse::<usize>().unwrap());

    let mut seeds: Vec<usize> = seeds.collect();

    almanac.next();

    let mut mappings = Vec::new();
    for line in almanac {
        if !line.ends_with("map:") {
            let mut numbers = line.split(' ').map(|s| s.parse().unwrap());
            mappings.push(Mapper {
                destination_rs: numbers.next().unwrap(),
                source_rs: numbers.next().unwrap(),
                range_length: numbers.next().unwrap(),
            });
        } else {
            seeds.iter_mut().for_each(|seed| {
                let mut x = mappings.iter().filter_map(|mapping| mapping.map(*seed));
                *seed = x.next().unwrap_or(*seed);
            });
            mappings = Vec::new();
        }
    }

    seeds.iter_mut().for_each(|seed| {
        let mut x = mappings.iter().filter_map(|mapping| mapping.map(*seed));
        *seed = x.next().unwrap_or(*seed);
    });

    seeds.iter().min().unwrap().to_owned()
}

#[derive(Clone)]
struct SeedsRange {
    start: usize,
    length: usize,
}

impl SeedsRange {
    fn split_map(&self, map: &Mapper) -> Option<(SeedsRange, Option<SeedsRange>)> {
        let start = map.map(self.start)?;
        if self.get_seeds_range_end() < map.get_source_end() {
            let mut x = self.clone();
            x.start = start;
            Some((x, None))
        } else {
            let split = SeedsRange {
                start: map.get_source_end(),
                length: self.get_seeds_range_end() - map.get_source_end(),
            };
            let mut x = self.clone();
            x.start = start;
            x.length -= split.length;
            Some((x, Some(split)))
        }
    }
    fn get_seeds_range_end(&self) -> usize {
        self.start + self.length - 1
    }
}

pub fn part02(file_path: &str) -> usize {
    let file = std::fs::read_to_string(file_path).unwrap();
    let mut almanac = file.lines().filter(|line| !line.is_empty());

    let mut seeds_ranges_data = almanac
        .next()
        .unwrap()
        .trim_start_matches("seeds: ")
        .split(' ')
        .map(|s| s.parse::<usize>().unwrap());

    let mut seeds_ranges: Vec<SeedsRange> = Vec::new();

    while let Some(start) = seeds_ranges_data.next() {
        let length = seeds_ranges_data.next().unwrap();
        seeds_ranges.push(SeedsRange { start, length });
    }

    almanac.next();

    let mut mappings = Vec::new();
    for line in almanac {
        if !line.ends_with("map:") {
            let mut numbers = line.split(' ').map(|s| s.parse().unwrap());
            mappings.push(Mapper {
                destination_rs: numbers.next().unwrap(),
                source_rs: numbers.next().unwrap(),
                range_length: numbers.next().unwrap(),
            });
        } else {
            seeds_ranges = split_and_map(seeds_ranges, &mappings);
            mappings = Vec::new();
        }
    }

    seeds_ranges = split_and_map(seeds_ranges, &mappings);

    let seeds = seeds_ranges.iter().map(|seeds_ranges| seeds_ranges.start);
    seeds.min().unwrap().to_owned()
}

fn split_and_map(mut seeds_ranges: Vec<SeedsRange>, mappings: &Vec<Mapper>) -> Vec<SeedsRange> {
    let mut splits = Vec::new();
    seeds_ranges.iter_mut().for_each(|seed| {
        let split_map = mappings.iter().find_map(|mapping| seed.split_map(mapping));
        if let Some((maped, split)) = split_map {
            *seed = maped;
            if split.is_some() {
                splits.push(split.unwrap());
            }
        }
    });

    if !splits.is_empty() {
        seeds_ranges.append(&mut split_and_map(splits, mappings));
    }

    seeds_ranges
}
