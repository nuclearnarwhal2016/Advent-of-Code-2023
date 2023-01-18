// Rock Paper Scissor
// A     B    C
// X     Y    Z
// 1     2    3
// LOSES
// B     C    A
// 2     3    1
// WINS
// C     A    B

pub fn part_one(input: &str) -> String {
    input
        .split("\n")
        .map(|line|{
            let choices = line.split(" ").collect::<Vec<&str>>();
            match choices.get(0){
                Some(expr) => expr,
                None => expr,
            }

        })
    .sum::<u32>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_part() {
        assert_eq!(result, 4);
    }
}
