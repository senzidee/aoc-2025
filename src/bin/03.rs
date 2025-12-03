advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input.split('\n').fold(0, |acc, line| {
            let mut a= 0;
            let mut b = 0;
            let l = line.len() - 1;
            for (position, c) in line.chars().enumerate() {
                let digit = c.to_digit(10).unwrap();
                if a < digit && position < l {
                    a = digit;
                    b = 0;
                } else if b < digit && position <= l {
                    b = digit;
                }
            }
            acc + (a * 10 + b) as u64
        }))
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        input.split('\n').fold(0, |acc, line| {
            let mut a1= 0;
            let mut a2 = 0;
            let mut a3 = 0;
            let mut a4 = 0;
            let mut a5 = 0;
            let mut a6 = 0;
            let mut a7 = 0;
            let mut a8 = 0;
            let mut a9 = 0;
            let mut a10 = 0;
            let mut a11 = 0;
            let mut a12 = 0;
            let l = line.len();
            for (position, c) in line.chars().enumerate() {
                let digit = c.to_digit(10).unwrap();
                if a1 < digit && position <= l - 12 {
                    a1 = digit;
                    a2 = 0;
                    a3 = 0;
                    a4 = 0;
                    a5 = 0;
                    a6 = 0;
                    a7 = 0;
                    a8 = 0;
                    a9 = 0;
                    a10 = 0;
                    a11 = 0;
                    a12 = 0;
                } else if a2 < digit && position <= l - 11 {
                    a2 = digit;
                    a3 = 0;
                    a4 = 0;
                    a5 = 0;
                    a6 = 0;
                    a7 = 0;
                    a8 = 0;
                    a9 = 0;
                    a10 = 0;
                    a11 = 0;
                    a12 = 0;
                } else if a3 < digit && position <= l - 10 {
                    a3 = digit;
                    a4 = 0;
                    a5 = 0;
                    a6 = 0;
                    a7 = 0;
                    a8 = 0;
                    a9 = 0;
                    a10 = 0;
                    a11 = 0;
                    a12 = 0;
                } else if a4 < digit && position <= l - 9 {
                    a4 = digit;
                    a5 = 0;
                    a6 = 0;
                    a7 = 0;
                    a8 = 0;
                    a9 = 0;
                    a10 = 0;
                    a11 = 0;
                    a12 = 0;
                } else if a5 < digit && position <= l - 8 {
                    a5 = digit;
                    a6 = 0;
                    a7 = 0;
                    a8 = 0;
                    a9 = 0;
                    a10 = 0;
                    a11 = 0;
                    a12 = 0;
                } else if a6 < digit && position <= l - 7 {
                    a6 = digit;
                    a7 = 0;
                    a8 = 0;
                    a9 = 0;
                    a10 = 0;
                    a11 = 0;
                    a12 = 0;
                } else if a7 < digit && position <= l - 6 {
                    a7 = digit;
                    a8 = 0;
                    a9 = 0;
                    a10 = 0;
                    a11 = 0;
                    a12 = 0;
                } else if a8 < digit && position <= l - 5 {
                    a8 = digit;
                    a9 = 0;
                    a10 = 0;
                    a11 = 0;
                    a12 = 0;
                } else if a9 < digit && position <= l - 4 {
                    a9 = digit;
                    a10 = 0;
                    a11 = 0;
                    a12 = 0;
                } else if a10 < digit && position <= l - 3 {
                    a10 = digit;
                    a11 = 0;
                    a12 = 0;
                } else if a11 < digit && position <= l - 2 {
                    a11 = digit;
                    a12 = 0;
                } else if a12 < digit && position <= l - 1 {
                    a12 = digit;
                }
            }
            let concatenate = format!("{}{}{}{}{}{}{}{}{}{}{}{}", a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11, a12);
            acc + concatenate.parse::<u64>().unwrap()
        }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
