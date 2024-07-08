pub fn part01(file_path: &str) -> usize {
    let file = std::fs::read_to_string(file_path).unwrap();
    let dig_plan: Vec<Dig> = file
        .lines()
        .map(|line| {
            let mut line = line.split(' ');
            let direction = match line.next().unwrap() {
                "U" => Direction::UP,
                "D" => Direction::DOWN,
                "R" => Direction::RIGHT,
                "L" => Direction::LEFT,
                _ => unreachable!(),
            };
            let number = line.next().unwrap().parse().unwrap();

            Dig { direction, number }
        })
        .collect();

    let mut dig_points = Vec::new();

    let mut current_pos = Point { x: 0, y: 0 };
    for dig in dig_plan {
        match dig.direction {
            Direction::UP => {
                for _ in 0..dig.number {
                    current_pos.y = current_pos.y - 1;
                    dig_points.push(Point {
                        x: current_pos.x,
                        y: current_pos.y,
                    });
                }
            }
            Direction::DOWN => {
                for _ in 0..dig.number {
                    current_pos.y = current_pos.y + 1;
                    dig_points.push(Point {
                        x: current_pos.x,
                        y: current_pos.y,
                    });
                }
            }
            Direction::LEFT => {
                for _ in 0..dig.number {
                    current_pos.x = current_pos.x - 1;
                    dig_points.push(Point {
                        x: current_pos.x,
                        y: current_pos.y,
                    });
                }
            }
            Direction::RIGHT => {
                for _ in 0..dig.number {
                    current_pos.x = current_pos.x + 1;
                    dig_points.push(Point {
                        x: current_pos.x,
                        y: current_pos.y,
                    });
                }
            }
        }
    }

    let max_x = dig_points.iter().max_by_key(|p| p.x).unwrap().x;
    let max_y = dig_points.iter().max_by_key(|p| p.y).unwrap().y;
    let min_x = dig_points.iter().min_by_key(|p| p.x).unwrap().x;
    let min_y = dig_points.iter().min_by_key(|p| p.y).unwrap().y;

    let offset_x = min_x.abs();
    let offset_y = min_y.abs();
    for point in dig_points.iter_mut() {
        point.x += offset_x;
        point.y += offset_y;
    }

    let mut terrain_map =
        vec![vec!['.'; (max_x + offset_x + 1) as usize]; (max_y + offset_y + 1) as usize];
    for dig_point in dig_points {
        terrain_map[dig_point.y as usize][dig_point.x as usize] = '#';
    }

    let start = terrain_map[1]
        .iter()
        .enumerate()
        .skip(1)
        .find(|(x, c)| **c == '.' && terrain_map[1][x - 1] == '#')
        .map(|(x, _)| Point { x: x as i64, y: 1 })
        .unwrap();

    terrain_map = digout(start, terrain_map);

    terrain_map.iter().flatten().filter(|c| **c == '#').count()
}

fn digout(start: Point, mut terrain_map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let max_x = terrain_map[0].len() - 1;
    let max_y = terrain_map.len() - 1;
    let x = start.x as usize;
    let y = start.y as usize;
    terrain_map[y][x] = '#';

    for p in get_adjacent_points(start, max_x as i64, max_y as i64) {
        let x = p.x as usize;
        let y = p.y as usize;
        if terrain_map[y][x] == '.' {
            terrain_map = digout(p, terrain_map);
        }
    }

    terrain_map
}

fn get_adjacent_points(point: Point, max_x: i64, max_y: i64) -> Vec<Point> {
    let x = point.x;
    let y = point.y;

    let adjacent_pos_offset = [[0, -1], [0, 1], [-1, 0], [1, 0]];
    let mut adjacent_pos = Vec::new();

    for [x_offset, y_offset] in adjacent_pos_offset.iter() {
        let x = x + x_offset;
        let y = y + y_offset;
        if !(x < 0 || y < 0 || x > max_x || y > max_y) {
            let x = x.try_into().unwrap();
            let y = y.try_into().unwrap();
            adjacent_pos.push(Point { x, y });
        }
    }

    adjacent_pos
}

#[derive(Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}

struct Dig {
    direction: Direction,
    number: usize,
}

enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}
