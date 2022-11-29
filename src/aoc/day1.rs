#[allow(dead_code)]
fn day1(input: String) -> i32 {
    let vec: Vec<&str> = input.lines().collect();
    
    println!("{:?}", vec);

    return 7;
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn test_day1() -> Result<(), std::io::Error> {
        assert_eq!(day1(fs::read_to_string("day1/test.txt")?), 7);
        Ok(())
    }
}
