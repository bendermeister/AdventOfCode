fn resolve_operand(register: &[usize], op: usize) -> usize {
    match op {
        w @ 0..=3 => w,
        4 => register[0],
        5 => register[1],
        6 => register[2],
        _ => panic!("combo operands higher than 6 are not allowed"),
    }
}

fn next(ip: &mut usize, program: &[usize], register: &mut [usize]) -> Option<usize> {
    while *ip < program.len() {
        let instruction = program[*ip];
        let operand = program[*ip + 1];
        *ip += 2;
        match instruction {
            // ADV combo
            0 => register[0] = register[0] / (1 << resolve_operand(register, operand)),
            // BXL literal
            1 => register[1] = register[1] ^ operand,
            // BST combo
            2 => register[1] = resolve_operand(register, operand) % 8,
            // JNZ literal
            3 => {
                if register[0] != 0 {
                    *ip = operand;
                }
            }
            // BXC _
            4 => register[1] = register[1] ^ register[2],
            // OUT combo
            5 => return Some(resolve_operand(register, operand) % 8),
            // BDV combo
            6 => register[1] = register[0] / (1 << resolve_operand(register, operand)),
            // CDV combo
            7 => register[2] = register[0] / (1 << resolve_operand(register, operand)),
            _ => panic!("instructions should be numbers in range 0..8"),
        }
    }
    return None;
}

fn main() {
    #[allow(unused)]
    let input = "
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
";

    let input = "
Register A: 33024962
Register B: 0
Register C: 0

Program: 2,4,1,3,7,5,1,5,0,3,4,2,5,5,3,0
    ";

    let mut iter = input.trim().split("\n\n");
    let register = iter.next().unwrap();

    let program = iter.next().unwrap();
    let program = &program[8..];
    let program: Vec<usize> = program
        .trim()
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect();

    let mut register: Vec<usize> = register
        .trim()
        .lines()
        .map(|l| l[11..].trim().parse().unwrap())
        .collect();
    assert!(iter.next() == None);

    // Part 1
    let mut ip = 0;

    while let Some(val) = next(&mut ip, &program, &mut register) {
        print!("{val},");
    }
    println!();

    // Part 2
    let mut stack = Vec::new();
    let mut new_stack = Vec::new();
    stack.push(0);

    // Note: this might only work for my input it is depended on the a
    // = a << 3 each round if a is depended on state of b or c and
    // their state is depended on the last a this would not be
    // possible
    for n in program.iter().rev() {
        while stack.len() > 0 {
            let candidate = stack.pop().unwrap() * 8;
            for i in 0..8 {
                register[0] = candidate + i;
                register[1] = 0;
                register[2] = 0;
                ip = 0;

                if let Some(m) = next(&mut ip, &program, &mut register) {
                    if m == *n {
                        new_stack.push(candidate + i);
                    }
                }
            }
        }
        let tmp = stack;
        stack = new_stack;
        new_stack = tmp;
        new_stack.clear();
    }

    let mut min = stack.pop().unwrap();
    while let Some(next) = stack.pop() {
        if min > next {
            min = next;
        }
    }

    println!("Solution 2: {min}");
}
