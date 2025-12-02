advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    Some(input.split(',').fold(0, |mut acc, line| {
        let range: Vec<u64> = line.split('-').map(|n| n.parse::<u64>().unwrap()).collect();
        for n in range[0]..=range[1] {
            let s = n.to_string();
            if s.len() % 2 != 0 {
                continue;
            }
            let (left, right) = s.split_at(s.len() / 2);
            if left == right {
                acc += n;
            }
        }
        acc
    }))
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(input.split(',').fold(0, |mut acc, line| {
        let range: Vec<u64> = line.split('-').map(|n| n.parse::<u64>().unwrap()).collect();
        for n in range[0]..=range[1] {
            let s = n.to_string();
            for i in 2..=s.len() {
                if s.len() % i != 0 {
                    continue;
                }
                let chunk_size = s.len() / i;
                let parts: Vec<&str> = s
                    .as_bytes()
                    .chunks(chunk_size)
                    .map(|chunk| std::str::from_utf8(chunk).unwrap())
                    .collect();
                if parts.windows(2).all(|w| w[0] == w[1]) {
                    acc += n;
                    break;
                }
            }
        }
        acc
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
