use std::{collections::HashSet, time::Duration, vec};

use rhai::Instant;

// Johnny Pipewalker
fn main() {
    // PART1
    let mut start = std::time::Instant::now();
    let bytes = include_bytes!("../../inputs/a10.txt").to_vec();
    let field = Field::from(bytes);
    let (steps, outer_bounds) = field.find_loop();
    println!(
        "Part 1 steps: {}. Found after {}µs",
        (steps as f32 / 2.0),
        std::time::Instant::now().duration_since(start).as_millis()
    );
}

#[derive(Debug, PartialEq)]
enum FieldKind {
    VPipe,
    HPipe,
    BendNorthEast,
    BendNorthWest,
    BendSouthEast,
    BendSouthWest,
    Ground,
    Start,
}

impl From<&u8> for FieldKind {
    fn from(value: &u8) -> Self {
        match value {
            b'|' => Self::VPipe,
            b'-' => Self::HPipe,
            b'L' => Self::BendNorthEast,
            b'J' => Self::BendNorthWest,
            b'F' => Self::BendSouthEast,
            b'7' => Self::BendSouthWest,
            b'S' => Self::Start,
            _ => Self::Ground,
        }
    }
}

impl FieldKind {
    fn new_direction(&self, direction_from: Direction) -> Option<Direction> {
        match (&self, direction_from) {
            (Self::VPipe, Direction::South | Direction::North) => Some(direction_from),
            (Self::HPipe, Direction::Easth | Direction::West) => Some(direction_from),
            // South into F or North into L -> East
            (Self::BendNorthEast, Direction::South) | (Self::BendSouthEast, Direction::North) => {
                Some(Direction::Easth)
            }
            // West into L or East into J -> North
            (Self::BendNorthEast, Direction::West) | (Self::BendNorthWest, Direction::Easth) => {
                Some(Direction::North)
            }
            // South into J or North into 7 -> West
            (Self::BendNorthWest, Direction::South) | (Self::BendSouthWest, Direction::North) => {
                Some(Direction::West)
            }
            // West into F or East into 7 -> South
            (Self::BendSouthEast, Direction::West) | (Self::BendSouthWest, Direction::Easth) => {
                Some(Direction::South)
            }
            _ => None,
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    North,
    Easth,
    South,
    West,
}

impl Direction {
    fn step(&self, x: usize, y: usize) -> Option<(usize, usize)> {
        match &self {
            Self::North => {
                if x > 0 {
                    Some((x - 1, y))
                } else {
                    None
                }
            }
            Self::Easth => Some((x, y + 1)),
            Self::South => Some((x + 1, y)),
            Self::West => {
                if y > 0 {
                    Some((x, y - 1))
                } else {
                    None
                }
            }
        }
    }
}

#[derive(Debug)]
struct Field(Vec<Vec<FieldKind>>);

impl From<Vec<u8>> for Field {
    fn from(value: Vec<u8>) -> Self {
        let v: Vec<Vec<FieldKind>> = value
            .split(|x| x == &b'\n')
            .map(|x| x.iter().map(|x| FieldKind::from(x)).collect())
            .collect();
        Field(v)
    }
}

impl Field {
    fn find_start(&self) -> (usize, usize) {
        for (index, line) in self.0.iter().enumerate() {
            for (line_index, field) in line.iter().enumerate() {
                if let FieldKind::Start = field {
                    return (index, line_index);
                }
            }
        }
        panic!("No Start found!");
    }
    fn find_loop(&self) -> (u32, HashSet<(usize, usize)>) {
        let (start_x, start_y) = self.find_start();
        let mut fields: HashSet<(usize, usize)> = HashSet::new();
        // do first step outside loop
        for start_direction in [
            Direction::North,
            Direction::South,
            Direction::Easth,
            Direction::West,
        ] {
            let (mut x, mut y) = (start_x, start_y);
            fields.insert((x, y));
            let mut steps = 0;
            let mut current_direction = start_direction;
            while steps == 0 || (x != start_x || y != start_y) {
                // Do next step
                if let Some((new_x, new_y)) = current_direction.step(x, y) {
                    (x, y) = (new_x, new_y);
                    steps += 1;
                    fields.insert((x, y));
                } else {
                    println!(
                        "Cannot take step to {:?} at ({},{}). Array bounds",
                        self.0[x][y], x, y
                    );
                    break;
                }
                // if again at start, break
                if steps > 0 && self.0[x][y] == FieldKind::Start {
                    break;
                }
                // if not, find new direction from current field
                if let Some(direction) = self.0[x][y].new_direction(current_direction) {
                    current_direction = direction;
                } else {
                    println!(
                        "Cannot take step to {:?} at ({},{}), moving: {:?}",
                        self.0[x][y], x, y, current_direction
                    );
                    break;
                }
            }
            if steps > 0 && x == start_x && y == start_y {
                println!(
                    "Found loop at ({},{}) in direction: {:?}, steps: {}",
                    x, y, start_direction, steps
                );
                return (steps, fields);
            } else {
                println!(
                    "Not able to find loop at ({},{}) in direction {:?}!",
                    x, y, current_direction
                );
            }
        }
        (0, fields)
    }
    fn find_inner_fields(&self) -> usize {
        let (_, outer_bounds) = self.find_loop();
        let mut inner_fields: HashSet<(usize, usize)> = HashSet::new();
        for (field_x, field_y) in &outer_bounds {
            for direction in [
                Direction::North,
                Direction::South,
                Direction::Easth,
                Direction::West,
            ] {
                // println!("Field {},{} direction {:?}", field_x, field_y, direction);
                let (mut x, mut y) = (*field_x, *field_y);
                let mut collector: HashSet<(usize, usize)> = HashSet::new();
                while (x <= self.0.len() && y <= self.0[0].len()) {
                    if let Some((x_new, y_new)) = direction.step(x, y) {
                        (x,y) = (x_new, y_new);
                    } else {
                        break; 
                    }
                    // println!("{},{}", x,y);
                    if outer_bounds.contains(&(x, y)) {
                        if collector.len() > 0 {
                            println!("inserting {} fields", collector.len());
                        }
                        for element in collector {
                            inner_fields.insert(element);
                        }
                        break;
                    } else {
                        collector.insert((x, y));
                    }
                    // bounds check
                }
            }
        }
        dbg!(&inner_fields);
        inner_fields.len()
    }
}

mod test {
    use crate::Field;

    // #[test]
    // fn part1() {
    //     let bytes = include_bytes!("../../inputs/a10_test.txt").to_vec();
    //     let field = Field::from(bytes);
    //     let (steps, _) = field.find_loop();
    //     assert_eq!(8, steps / 2);
    // }
    #[test]
    fn part2() {
        let bytes = include_bytes!("../../inputs/a10_test2.txt").to_vec();
        let field = Field::from(bytes);
        let inner_fields = field.find_inner_fields();
        assert_eq!(inner_fields, 10);
    }
}
