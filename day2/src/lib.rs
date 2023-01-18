// Rock Paper Scissor
// A     B    C
// X     Y    Z
// 1     2    3
// LOSES
// B     C    A
// 2     3    1
// WINS
// C     A    B
// A, X => draw
// A, Y => lose 
// A, Z => win
// 
pub fn win_or_lose(first: &str, second: &str) -> bool {
    false
}

pub fn part_one(input: &str) -> String {
    input
        .split("\n")
        .map(|line| {
            let choices = line.split(" ").collect::<Vec<&str>>();
            let mut sum = 0;
            match choices.get(1).unwrap() {
                &"X" => sum += 1,
                &"Y" => sum += 2,
                &"Z" => sum += 3,
                &_ => sum += 0,
            };
            sum
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_part() {
        let result = "A Y
B X
C Z";
        assert_eq!(part_one(result), "15");
    }
}
