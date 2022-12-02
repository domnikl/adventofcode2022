use std::collections::HashMap;

fn play(shapes: Vec<&str>) -> i32 {
    let signs = HashMap::from([
        ("A", 1), // rock
        ("B", 2), // paper
        ("C", 3), // scissors
        ("X", 1), // rock
        ("Y", 2), // paper
        ("Z", 3), // scissors
    ]);

    let opponent = signs.get(&*shapes[0]).expect("Not a valid symbol");
    let me = signs.get(&*shapes[1]).expect("Not a valid symbol");

    let points = if me == opponent {
        me + 3
    } else if shapes == vec!["C", "X"] || shapes == vec!["A", "Y"] || shapes == vec!["B", "Z"] {
        me + 6
    } else {
        me + 0
    };

    return points;
}

#[allow(dead_code)]
fn day2(input: String) -> i32 {
    return input
        .lines()
        .map(|l| play(l.split(" ").collect()))
        .sum();
}

#[allow(dead_code)]
fn day2_part2(_input: String) -> i32 {
    return 1;
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn test_day2() -> Result<(), std::io::Error> {
        assert_eq!(day2(fs::read_to_string("input/day2.txt")?), 15);
        //println!("day2 part 1: {:?}", day2(fs::read_to_string("input/day2_full.txt")?));
        Ok(())
    }

    #[test]
    fn test_day2_part2() -> Result<(), std::io::Error> {
        assert_eq!(day2_part2(fs::read_to_string("input/day2.txt")?), 12);
        println!("day 2 part 2: {:?}", day2_part2(fs::read_to_string("input/day2_full.txt")?));
        Ok(())
    }
}
