use std::{collections::{HashSet, HashMap}, time::Duration, vec};

use rhai::Instant;

// Johnny Pipewalker
fn main() {
    // PART1
    let mut start = std::time::Instant::now();
    let bytes = include_bytes!("../../inputs/a10.txt").to_vec();
    let field = Field::from(bytes);
    let (steps, _, _) = field.find_loop();
    println!(
        "Part 1 steps: {}. Found after {}µs",
        (steps as f32 / 2.0),
        std::time::Instant::now().duration_since(start).as_millis()
    );
    let mut start = std::time::Instant::now();
    let count = field.count_inner_fields();
    println!(
        "Part 2 inner fields: {}. Found after {}µs",
        count,
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
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
    fn find_loop(
        &self,
    ) -> (
        u32,
        HashMap<(usize, usize), Direction>,
        HashSet<(usize, usize)>,
    ) {
        let (start_x, start_y) = self.find_start();
        // do first step outside loop
        for start_direction in [
            Direction::North,
            Direction::South,
            Direction::Easth,
            Direction::West,
        ] {
            let (mut x, mut y) = (start_x, start_y);
            let mut noso_fields: HashMap<(usize, usize), Direction> = HashMap::new();
            let mut all_fields: HashSet<(usize, usize)> = HashSet::new();
            let mut steps = 0;
            let mut current_direction = start_direction;
            if [Direction::North, Direction::South].contains(&current_direction) {
                noso_fields.insert((x, y), current_direction);
                println!("insert@{x},{y} - start")
            }
            all_fields.insert((x, y));
            while steps == 0 || (x != start_x || y != start_y) {
                // Do next step
                if let Some((new_x, new_y)) = current_direction.step(x, y) {
                    (x, y) = (new_x, new_y);
                    steps += 1;
                    all_fields.insert((x, y));
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
                    if [Direction::North, Direction::South].contains(&current_direction) {
                        noso_fields.insert((x, y), current_direction);
                        println!("insert@{x},{y} - {current_direction:?}")
                    }
                    current_direction = direction;
                    if [Direction::North, Direction::South].contains(&current_direction) {
                        noso_fields.insert((x, y), current_direction);
                        println!("insert@{x},{y} - {current_direction:?}")
                    }
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
                return (steps, noso_fields, all_fields);
            } else {
                println!(
                    "Not able to find loop at ({},{}) in direction {:?}!",
                    x, y, current_direction
                );
            }
        }
        (0, HashMap::new(), HashSet::new())
    }
    fn count_inner_fields(&self) -> usize {
        let (_, noso_fields, all_fields) = self.find_loop();
        println!("{:?}", noso_fields);
        let mut count = 0;
        for (x, line) in self.0.iter().enumerate() {
            let mut inside = false;
            let mut last_direction: Option<Direction> = None;
            for (y, _) in line.iter().enumerate() {
                if noso_fields.contains_key(&(x, y)) {
                    if let Some(last) = last_direction {
                        if last != noso_fields[&(x,y)] {
                            inside = !inside;
                            last_direction = Some(noso_fields[&(x,y)]);
                        }
                    } else {
                        inside = !inside;
                        last_direction = Some(noso_fields[&(x,y)]);
                    }
                    println!("switch@{x},{y}");
                } else if inside && !all_fields.contains(&(x, y)) {
                    println!("found@{x},{y}");
                    count += 1;
                }
            }
        }
        count
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
        let inner_fields = field.count_inner_fields();
        assert_eq!(inner_fields, 10);
    }
}
