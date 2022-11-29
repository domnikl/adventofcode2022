fn input_to_vec(input: String) -> Vec<u16> {
    let vec: Vec<&str> = input.lines().collect();
    let numbers: Vec<u16> = vec.iter().map(|x| x.parse::<u16>().unwrap()).collect();

    return numbers;
}

fn count_increasing(a: Vec<u16>) -> u16 {
    let mut previous: Option<u16> = None;
    let mut count: u16 = 0;

    for x in a {
        if previous.is_some() && x > previous.unwrap() {
            count += 1;
        }

        previous = Some(x);
    }
    
    return count;   
}

#[allow(dead_code)]
fn day0(input: String) -> u16 {
    let numbers = input_to_vec(input);

    return count_increasing(numbers);
}

#[allow(dead_code)]
fn day0_part2(input: String) -> u16 {
    let numbers = input_to_vec(input);
    let windows: Vec<u16> = numbers.windows(2).map(|w| w.iter().fold(0, |mut sum, &val| { sum += val; sum})).collect();

    return count_increasing(windows);
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn test_day0() -> Result<(), std::io::Error> {
        assert_eq!(day0(fs::read_to_string("day0/test.txt")?), 7);
        Ok(())
    }

     #[test]
    fn test_day0_part2() -> Result<(), std::io::Error> {
        assert_eq!(day0_part2(fs::read_to_string("day0/test.txt")?), 5);
        Ok(())
    }
}
