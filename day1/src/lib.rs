pub fn part_one(file: &str) -> String {
    file.split("\n\n")
        .map(|chunk| {
            chunk
                .split("\n")
                .map(|cal| cal.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max()
        .unwrap()
        .to_string()
}

pub fn part_two(file: &str) -> String {
    let mut result: Vec<u32> = file
        .split("\n\n")
        .map(|chunk| {
            chunk
                .split("\n")
                .map(|cal| cal.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<_>>();
    result.sort_by(|a, b| b.cmp(a));
    result.get(0..3).unwrap().iter().sum::<u32>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const RESULT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
    #[test]
    fn first_part() {
        assert_eq!(part_one(RESULT), "24000");
    }

    #[test]
    fn second_part() {
        assert_eq!(part_two(RESULT), "45000");
    }
}
