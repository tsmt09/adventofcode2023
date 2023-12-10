fn main() {
    // PART1
    let mut start = std::time::Instant::now();
    let bytes = include_bytes!("../../inputs/a9.txt");
    let structure = parse(bytes, false);
    println!(
        "parsing after {} µs",
        std::time::Instant::now().duration_since(start).as_micros()
    );
    let p1_result = structure
        .iter()
        .map(|x| derive(x))
        .reduce(|acc, e| acc + e)
        .unwrap();
    println!(
        "P1: {}. done after {} µs",
        p1_result,
        std::time::Instant::now().duration_since(start).as_micros()
    );
    // PART2
    println!("--- PART 2 ---");
    start = std::time::Instant::now();
    let structure = parse(bytes, true);
    println!(
        "parsing after {} µs",
        std::time::Instant::now().duration_since(start).as_micros()
    );
    let p2_result = structure
        .iter()
        .map(|x| derive(x))
        .reduce(|acc, e| acc + e)
        .unwrap();
    println!(
        "P2: {}. done after {} µs",
        p2_result,
        std::time::Instant::now().duration_since(start).as_micros()
    );
}

fn derive(line: &[i64]) -> i64 {
    let mut iter = line.iter().peekable();
    let mut current = iter.next().expect("line should not be empty");
    let mut next_line: Vec<i64> = vec![];
    let mut allzero = true;
    while iter.peek().is_some() {
        let next = iter.peek().unwrap();
        let num = *next - current;
        if num != 0 {
            allzero = false;
        }
        next_line.push(*next - current);
        current = iter.next().unwrap();
    }
    if allzero {
        *line.last().unwrap()
    } else {
        let derived = derive(&next_line);
        line.last().unwrap() + derived
    }
}

fn parse(input: &[u8], rev: bool) -> Vec<Vec<i64>> {
    input
        .split(|x| x == &b'\n')
        .map(|x| {
            // inner Vec<&str>
            let mut vec: Vec<i64> = x
                .split(|x| x == &b' ')
                .map(|x| std::str::from_utf8(x).unwrap().parse::<i64>().unwrap())
                .collect();
            if rev {
                vec.reverse();
            }
            vec
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::derive;
    use super::parse;
    #[test]
    fn part1() {
        let bytes = include_bytes!("../../inputs/a9_test.txt");
        let structure = parse(bytes, false);
        let r = structure
            .iter()
            .map(|line| derive(line))
            .reduce(|acc, e| acc + e)
            .unwrap();
        assert_eq!(r, 114);
    }
    #[test]
    fn part2() {
        let bytes = include_bytes!("../../inputs/a9_test.txt");
        let structure = parse(bytes, true);
        let r = structure
            .iter()
            .map(|line| derive(line))
            .reduce(|acc, e| acc + e)
            .unwrap();
        assert_eq!(r, 2);
    }
}
