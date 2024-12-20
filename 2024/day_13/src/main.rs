mod part_1 {
    #[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl std::ops::Add for Point {
        type Output = Point;
        fn add(self, other: Point) -> Self::Output {
            Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    impl std::ops::Sub for Point {
        type Output = Point;
        fn sub(self, other: Point) -> Self::Output {
            Point {
                x: self.x - other.x,
                y: self.y - other.y,
            }
        }
    }

    impl Point {
        fn new(x: i32, y: i32) -> Point {
            Point { x, y }
        }
        #[allow(unused)]
        fn to_string(self) -> String {
            format!("({}, {})", self.x, self.y)
        }
        fn scalar_product(self, other: i32) -> Point {
            Point {
                x: self.x * other,
                y: self.y * other,
            }
        }
    }

    fn calc(prize: Point, a: Point, b: Point) -> Option<(i32, i32)> {
        let mut ac = 0;
        let mut ap = a.scalar_product(ac);

        let mut stack = Vec::new();

        while ap.x <= prize.x && ap.y <= prize.y {
            let bp = prize - ap;
            let bc = bp.x / b.x;
            if b.scalar_product(bc) == bp {
                stack.push((ac, bc));
            }

            ac += 1;
            ap = a.scalar_product(ac);
        }

        if stack.is_empty() {
            return None;
        }
        let mut min = stack[0];
        let cost = |(a, b)| a * 3 + b;
        let mut min_cost = cost(min);

        while stack.len() > 0 {
            let pair = stack.pop().unwrap();
            if min_cost > cost(pair) {
                min_cost = cost(pair);
                min = pair;
            }
        }
        return Some(min);
    }

    fn solve(s: &str) -> usize {
        let lines: Vec<&str> = s.lines().map(|l| l.trim()).collect();
        assert!(lines.len() == 3);
        let button_a: Vec<_> = lines[0][10..]
            .split(",")
            .map(|n| n.trim())
            .map(|n| &n[2..])
            .map(|n| n.parse::<i32>().unwrap())
            .collect();
        let button_b: Vec<_> = lines[1][10..]
            .split(",")
            .map(|n| n.trim())
            .map(|n| &n[2..])
            .map(|n| n.parse::<i32>().unwrap())
            .collect();
        let prize: Vec<_> = lines[2][7..]
            .split(",")
            .map(|n| n.trim())
            .map(|n| &n[2..])
            .map(|n| n.parse::<i32>().unwrap())
            .collect();
        assert!(button_a.len() == 2);
        assert!(button_b.len() == 2);
        assert!(prize.len() == 2);
        let a = Point::new(button_a[0], button_a[1]);
        let b = Point::new(button_b[0], button_b[1]);
        let p = Point::new(prize[0], prize[1]);

        // println!("A: {}", a.to_string());
        // println!("B: {}", b.to_string());
        // println!("P: {}", p.to_string());

        let solution = calc(p, a, b);
        match solution {
            None => {
                //println!("No solution");
                0
            }
            Some((a, b)) => {
                //println!("a: {a}, b: {b}, cost: {}", a * 3 + b);
                (a * 3 + b) as usize
            }
        }
    }

    pub fn run(input: &str) {
        let solution_1: usize = input.trim().split("\n\n").map(|c| solve(c)).sum();

        println!("Solution 1: {solution_1}");
    }
}

mod part_2 {
    #[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
    struct Point {
        x: isize,
        y: isize,
    }

    impl std::ops::Add for Point {
        type Output = Point;
        fn add(self, other: Point) -> Self::Output {
            Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    impl std::ops::Sub for Point {
        type Output = Point;
        fn sub(self, other: Point) -> Self::Output {
            Point {
                x: self.x - other.x,
                y: self.y - other.y,
            }
        }
    }

    impl Point {
        fn new(x: isize, y: isize) -> Point {
            Point { x, y }
        }
        #[allow(unused)]
        fn to_string(self) -> String {
            format!("({}, {})", self.x, self.y)
        }
        fn scalar_product(self, other: isize) -> Point {
            Point {
                x: self.x * other,
                y: self.y * other,
            }
        }
    }

    fn calc(p: Point, a: Point, b: Point) -> Option<(isize, isize)> {
        let mut stack = Vec::new();

        let ac = (p.x * b.y - p.y * b.x) / (a.x * b.y - a.y * b.x);
        let ap = a.scalar_product(ac);
        let bp = p - ap;
        let bc = bp.x / b.x;
        if b.scalar_product(bc) == bp {
            stack.push((ac, bc));
        }

        let ac = ac + 1;
        let ap = a.scalar_product(ac);
        let bp = p - ap;
        let bc = bp.x / b.x;
        if b.scalar_product(bc) == bp {
            stack.push((ac, bc));
        }

        let bc = (p.x * a.y - p.y * a.x) / (b.x * a.y - b.y * a.x);
        let bp = a.scalar_product(ac);
        let ap = p - bp;
        let ac = ap.x / a.x;
        if a.scalar_product(bc) == ap {
            stack.push((ac, bc));
        }

        let bc = bc + 1;
        let bp = a.scalar_product(ac);
        let ap = p - bp;
        let ac = ap.x / a.x;
        if a.scalar_product(bc) == ap {
            stack.push((ac, bc));
        }

        if stack.is_empty() {
            return None;
        }
        let mut min = stack[0];
        let cost = |(a, b)| a * 3 + b;
        let mut min_cost = cost(min);

        while stack.len() > 0 {
            let pair = stack.pop().unwrap();
            if min_cost > cost(pair) {
                min_cost = cost(pair);
                min = pair;
            }
        }
        return Some(min);
    }

    fn solve(s: &str) -> usize {
        let lines: Vec<&str> = s.lines().map(|l| l.trim()).collect();
        assert!(lines.len() == 3);
        let button_a: Vec<_> = lines[0][10..]
            .split(",")
            .map(|n| n.trim())
            .map(|n| &n[2..])
            .map(|n| n.parse().unwrap())
            .collect();
        let button_b: Vec<_> = lines[1][10..]
            .split(",")
            .map(|n| n.trim())
            .map(|n| &n[2..])
            .map(|n| n.parse().unwrap())
            .collect();
        let prize: Vec<_> = lines[2][7..]
            .split(",")
            .map(|n| n.trim())
            .map(|n| &n[2..])
            .map(|n| n.parse().unwrap())
            .collect();
        assert!(button_a.len() == 2);
        assert!(button_b.len() == 2);
        assert!(prize.len() == 2);
        let a = Point::new(button_a[0], button_a[1]);
        let b = Point::new(button_b[0], button_b[1]);
        let p = Point::new(prize[0], prize[1]);
        let p = p + Point::new(10000000000000, 10000000000000);

        // println!("A: {}", a.to_string());
        // println!("B: {}", b.to_string());
        // println!("P: {}", p.to_string());

        let solution = calc(p, a, b);
        match solution {
            None => {
                //println!("No solution");
                0
            }
            Some((a, b)) => {
                //println!("a: {a}, b: {b}, cost: {}", a * 3 + b);
                (a * 3 + b) as usize
            }
        }
    }

    pub fn run(input: &str) {
        let solution: usize = input.trim().split("\n\n").map(|c| solve(c)).sum();

        println!("Solution 2: {solution}");
    }
}

fn main() {
    let input = "
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";
    let input = &std::fs::read_to_string("input").unwrap();
    part_1::run(input);
    part_2::run(input);
}
