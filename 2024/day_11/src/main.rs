mod solution_1 {

    use std::sync::mpsc;
    use std::thread;

    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    enum Child {
        One(usize),
        Two(usize, usize),
    }

    fn is_even(d: usize) -> bool {
        (d & 1) == 0
    }

    fn count_digits(d: usize) -> usize {
        let mut ten = 10;
        let mut n = 1;

        while d / ten != 0 {
            n += 1;
            ten *= 10;
        }
        n
    }

    fn apply_rule(s: usize) -> Child {
        if s == 0 {
            return Child::One(1);
        }
        let num_digits = count_digits(s);
        if is_even(num_digits) {
            let (upper, lower) = split_number(s, num_digits);
            return Child::Two(upper, lower);
        }
        Child::One(s * 2024)
    }

    type Stack = Vec<(usize, usize)>;

    fn blink_single(stack: &mut Stack, s: usize, g: usize) -> usize {
        if g == 0 {
            return 1;
        }
        let c = apply_rule(s);
        match c {
            Child::One(a) => {
                stack.push((a, g - 1));
            }
            Child::Two(a, b) => {
                stack.push((a, g - 1));
                stack.push((b, g - 1));
            }
        }
        return 0;
    }

    fn blink(stones: &Vec<usize>, generations: usize) -> usize {
        let mut stack = Stack::new();
        for s in stones {
            stack.push((*s, generations));
        }

        let mut sum = 0;
        while stack.len() > 0 && stack.len() < 22 {
            let (s, g) = stack.pop().unwrap();
            sum += blink_single(&mut stack, s, g);
        }

        let mut thread_handles = Vec::new();

        let (sender, receiver) = mpsc::channel();

        while stack.len() > 0 {
            let stone = stack.pop().unwrap();
            let mut stack = Stack::new();
            stack.push(stone);
            let sender = sender.clone();

            thread_handles.push(thread::spawn(move || {
                let mut sum = 0;
                while stack.len() > 0 {
                    let (s, g) = stack.pop().unwrap();
                    sum += blink_single(&mut stack, s, g);
                }

                sender.send(sum).unwrap();
            }))
        }

        for h in thread_handles {
            h.join().unwrap();
        }

        drop(sender);

        for val in receiver {
            sum += val;
        }

        return sum;
    }

    fn split_number(d: usize, n: usize) -> (usize, usize) {
        let mut ten = 10;
        for _ in 1..n / 2 {
            ten *= 10;
        }
        (d / ten, d % ten)
    }

    #[allow(unused)]
    pub fn run() {
        //let input = "125 17";
        let input = &std::fs::read_to_string("input").unwrap();
        let stones: Vec<usize> = input
            .trim()
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        let solution_1 = blink(&stones, 25);
        println!("Solution 1: {}", solution_1);

        let solution_2 = blink(&stones, 75);
        println!("Solution 2: {}", solution_2);
    }
}

fn main() {
    solution_1::run();
}
