fn parse_elf(input: String) -> Vec<u32> {
    return input.trim().split("\n").map(|x| x.parse::<u32>().unwrap()).collect();
}

fn input_to_vec(input: String) -> Vec<Vec<u32>> {
    let by_elf: Vec<&str> = input.split("\n\n").collect();
    let x = by_elf.iter().map(|x| parse_elf(x.to_string())).collect();

    return x
}

fn sum_by_elf(input: Vec<Vec<u32>>) -> Vec<u32> {
    return input.into_iter().map(|e| e.into_iter().reduce(|a, b| a + b).unwrap()).collect();
}

#[allow(dead_code)]
fn day1(input: String) -> u32 {
    let numbers = input_to_vec(input);
    let sums = sum_by_elf(numbers);

    return *sums.iter().max().unwrap();
}

#[allow(dead_code)]
fn day1_part2(input: String) -> u32 {
    let numbers = input_to_vec(input);
    let mut sums = sum_by_elf(numbers);

    sums.sort();
    sums.reverse();

    let top3 = &sums[0..3];

    return top3.to_vec().into_iter().reduce(|a,b| a+ b).unwrap();
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn test_day1() -> Result<(), std::io::Error> {
        assert_eq!(day1(fs::read_to_string("input/day1.txt")?), 24000);
        //println!("{:?}", day1(fs::read_to_string("input/day1_full.txt")?));

        Ok(())
    }

    #[test]
    fn test_day1_part2() -> Result<(), std::io::Error> {
        assert_eq!(day1_part2(fs::read_to_string("input/day1.txt")?), 45000);
        //println!("{:?}", day1_part2(fs::read_to_string("input/day1_full.txt")?));
        Ok(())
    }
}
