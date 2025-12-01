advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let mut counter: i64 = 50;
    Some(
        input
            .split('\n')
            .map(|line| {
                let direction = line.chars().next().unwrap();
                let amount: i64 = line[1..].parse().unwrap();
                if 'L' == direction {
                    counter -= amount;
                } else {
                    counter += amount;
                }
                while !(0..=99).contains(&counter) {
                    counter = parse_click(counter);
                }
                if counter == 0 {
                    return 1;
                }
                0
            })
            .sum::<u64>(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut counter: i64 = 50;
    Some(
        input
            .split('\n')
            .map(|line| {
                let mut clicks = 0;
                let direction = line.chars().next().unwrap();
                let amount: i64 = line[1..].parse().unwrap();
                if 'L' == direction {
                    counter -= amount;
                } else {
                    counter += amount;
                }
                while !(0..=99).contains(&counter) {
                    clicks += 1;
                    counter = parse_click(counter);
                }
                clicks
            })
            .sum::<u64>(),
    )
}

fn parse_click(mut click: i64) -> i64 {
    if click < 0 {
        click += 100;
    } else if click > 0 {
        click -= 100;
    }

    click
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
