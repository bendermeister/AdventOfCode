use std::str::FromStr;

#[derive(Debug)]
struct Equation {
    result: i64,
    nums: Vec<i64>,
}

impl FromStr for Equation {
    type Err = String;
    fn from_str(text: &str) -> Result<Equation, Self::Err> {
        let hands: Vec<_> = text.split(":").map(|l| l.trim()).collect();
        if hands.len() != 2 {
            return Err(String::from(
                "Equation does not follow form '<Left Hand>: <Right Hand>'",
            ));
        }

        let result = match hands[0].parse() {
            Ok(v) => v,
            _ => {
                return Err(format!(
                    "Could not parse left hand side of equation. Expected number got: '{}'",
                    hands[0]
                ));
            }
        };

        let mut nums = vec![];

        for num in hands[1].trim().split_whitespace().filter(|l| l.len() > 0) {
            let num = match num.parse() {
                Ok(v) => v,
                Err(_) => {
                    return Err(format!(
                        "Could not parse number in right hand side of equation"
                    ));
                }
            };
            nums.push(num);
        }

        Ok(Equation {
            result: result,
            nums: nums,
        })
    }
}

fn cat(a: i64, b: i64) -> i64 {
    let mut order = 10;
    while b / order != 0 {
        order *= 10;
    }

    a * order + b
}

impl Equation {
    fn is_solveable_helper(&self, current_sum: i64, num_index: usize) -> bool {
        if num_index >= self.nums.len() {
            return current_sum == self.result;
        }
        self.is_solveable_helper(current_sum + self.nums[num_index], num_index + 1)
            || self.is_solveable_helper(current_sum * self.nums[num_index], num_index + 1)
            || self.is_solveable_helper(cat(current_sum, self.nums[num_index]), num_index + 1)
    }
    pub fn is_solveable(&self) -> bool {
        return self.is_solveable_helper(0, 0);
    }
}

fn main() {
    //     let input = "
    // 190: 10 19
    // 3267: 81 40 27
    // 83: 17 5
    // 156: 15 6
    // 7290: 6 8 6 15
    // 161011: 16 10 13
    // 192: 17 8 14
    // 21037: 9 7 18 13
    // 292: 11 6 16 20
    // ";

    let input = std::fs::read_to_string("input").unwrap();
    let equations: i64 = input
        .trim()
        .lines()
        .map(|l| l.parse::<Equation>().unwrap())
        .filter(|e| e.is_solveable())
        .map(|e| e.result)
        .sum();
    dbg!(&equations);
}
