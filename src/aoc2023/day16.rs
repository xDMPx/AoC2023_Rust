pub fn part01(file_path: &str) -> usize {
    let file = std::fs::read_to_string(file_path).unwrap();
    let map: Vec<Vec<char>> = file.lines().map(|line| line.chars().collect()).collect();

    let beam = Beam {
        pos: (0, 0),
        direction: Direction::RIGHT,
    };

    let mut visited = vec![];
    traverse_map(beam, &map, &mut visited);

    visited.len()
}

fn traverse_map(beam: Beam, map: &Vec<Vec<char>>, visited: &mut Vec<(usize, usize)>) {
    let pos = beam.pos;
    let beams = advance_beam(beam, &map, visited);
    if !visited.contains(&pos) {
        visited.push(pos);
    }

    if let Some(beam) = beams.0 {
        traverse_map(beam, map, visited);
    }
    if let Some(beam) = beams.1 {
        traverse_map(beam, map, visited);
    }
}

enum Direction {
    LEFT,
    RIGHT,
    UP,
    DOWN,
}

struct Beam {
    pos: (usize, usize),
    direction: Direction,
}

fn advance_beam(
    beam: Beam,
    map: &Vec<Vec<char>>,
    visited: &mut Vec<(usize, usize)>,
) -> (Option<Beam>, Option<Beam>) {
    let map_width = map[0].len();
    let map_height = map.len();
    match map[beam.pos.0][beam.pos.1] {
        '.' => (move_beam_by_direction(beam, map[0].len(), map.len()), None),
        '|' => match beam.direction {
            Direction::LEFT | Direction::RIGHT => {
                if visited.contains(&beam.pos) {
                    return (None, None);
                }
                let beam1 = Beam {
                    pos: beam.pos,
                    direction: Direction::UP,
                };
                let beam2 = Beam {
                    pos: beam.pos,
                    direction: Direction::DOWN,
                };
                (
                    move_beam_by_direction(beam1, map_width, map_height),
                    move_beam_by_direction(beam2, map_width, map_height),
                )
            }
            _ => (move_beam_by_direction(beam, map_width, map_height), None),
        },
        '-' => match beam.direction {
            Direction::UP | Direction::DOWN => {
                if visited.contains(&beam.pos) {
                    return (None, None);
                }
                let beam1 = Beam {
                    pos: beam.pos,
                    direction: Direction::LEFT,
                };
                let beam2 = Beam {
                    pos: beam.pos,
                    direction: Direction::RIGHT,
                };
                (
                    move_beam_by_direction(beam1, map_width, map_height),
                    move_beam_by_direction(beam2, map_width, map_height),
                )
            }
            _ => (move_beam_by_direction(beam, map_width, map_height), None),
        },
        '/' => match beam.direction {
            Direction::UP => {
                let beam = Beam {
                    pos: beam.pos,
                    direction: Direction::RIGHT,
                };
                (move_beam_by_direction(beam, map_width, map_height), None)
            }
            Direction::DOWN => {
                let beam = Beam {
                    pos: beam.pos,
                    direction: Direction::LEFT,
                };
                (move_beam_by_direction(beam, map_width, map_height), None)
            }
            Direction::RIGHT => {
                let beam = Beam {
                    pos: beam.pos,
                    direction: Direction::UP,
                };
                (move_beam_by_direction(beam, map_width, map_height), None)
            }
            Direction::LEFT => {
                let beam = Beam {
                    pos: beam.pos,
                    direction: Direction::DOWN,
                };
                (move_beam_by_direction(beam, map_width, map_height), None)
            }
        },
        '\\' => match beam.direction {
            Direction::UP => {
                let beam = Beam {
                    pos: beam.pos,
                    direction: Direction::LEFT,
                };
                (move_beam_by_direction(beam, map_width, map_height), None)
            }
            Direction::DOWN => {
                let beam = Beam {
                    pos: beam.pos,
                    direction: Direction::RIGHT,
                };
                (move_beam_by_direction(beam, map_width, map_height), None)
            }
            Direction::RIGHT => {
                let beam = Beam {
                    pos: beam.pos,
                    direction: Direction::DOWN,
                };
                (move_beam_by_direction(beam, map_width, map_height), None)
            }
            Direction::LEFT => {
                let beam = Beam {
                    pos: beam.pos,
                    direction: Direction::UP,
                };
                (move_beam_by_direction(beam, map_width, map_height), None)
            }
        },
        _ => unreachable!(),
    }
}

fn move_beam_by_direction(beam: Beam, map_width: usize, map_height: usize) -> Option<Beam> {
    let pos = match beam.direction {
        Direction::RIGHT => {
            if beam.pos.1 + 1 < map_width {
                (beam.pos.0, beam.pos.1 + 1)
            } else {
                return None;
            }
        }
        Direction::LEFT => {
            if beam.pos.1 != 0 {
                (beam.pos.0, beam.pos.1 - 1)
            } else {
                return None;
            }
        }
        Direction::UP => {
            if beam.pos.0 != 0 {
                (beam.pos.0 - 1, beam.pos.1)
            } else {
                return None;
            }
        }
        Direction::DOWN => {
            if beam.pos.0 + 1 < map_height {
                (beam.pos.0 + 1, beam.pos.1)
            } else {
                return None;
            }
        }
    };

    Some(Beam {
        pos,
        direction: beam.direction,
    })
}
