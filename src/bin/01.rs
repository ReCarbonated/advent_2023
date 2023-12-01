advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;
    for line in input.lines() {
        let mut storage: Vec<u32> = Vec::new();
        for ch in line.chars() {
            if ch.is_ascii_digit() {
                storage.push(ch.to_digit(10).unwrap())
            }
        }
        let first = storage.first().unwrap();
        let last = storage.last().unwrap();
        sum += first * 10 + last;
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum = 0;
    for line in input.lines() {
        let new_line = line
            .replace("one", "one1one")
            .replace("two", "two2two")
            .replace("three", "three3three")
            .replace("four", "four4four")
            .replace("five", "five5five")
            .replace("six", "six6six")
            .replace("seven", "seven7seven")
            .replace("eight", "eight8eight")
            .replace("nine", "nine9nine");
        let mut storage: Vec<u32> = Vec::new();
        for ch in new_line.chars() {
            if ch.is_ascii_digit() {
                storage.push(ch.to_digit(10).unwrap())
            }
        }
        let first = storage.first().unwrap();
        let last = storage.last().unwrap();
        sum += first * 10 + last;
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
