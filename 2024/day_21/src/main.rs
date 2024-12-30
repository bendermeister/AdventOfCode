use std::collections::HashMap;

struct DirPad {
    transition: HashMap<char, HashMap<char, String>>,
}

impl DirPad {
    fn encode(&self, input: &str) -> String {
        let mut old = 'A';
        let mut result = String::new();
        for new in input.chars() {
            result.push_str(self.transition(old, new));
            old = new;
        }
        return result;
    }

    fn new() -> DirPad {
        let mut pad = DirPad {
            transition: HashMap::new(),
        };

        // ^
        {
            let mut map = HashMap::new();
            map.insert('^', String::from("A"));
            map.insert('A', String::from(">A"));
            map.insert('<', String::from("v<A"));
            map.insert('v', String::from("vA"));
            map.insert('>', String::from("v>A"));
            pad.transition.insert('^', map);
        }
        // A
        {
            let mut map = HashMap::new();
            map.insert('^', String::from("<A"));
            map.insert('A', String::from("A"));
            map.insert('<', String::from("v<<A"));
            map.insert('v', String::from("v<A"));
            map.insert('>', String::from("vA"));
            pad.transition.insert('A', map);
        }
        // <
        {
            let mut map = HashMap::new();
            map.insert('^', String::from(">^A"));
            map.insert('A', String::from(">>^A"));
            map.insert('<', String::from("A"));
            map.insert('v', String::from(">A"));
            map.insert('>', String::from(">>A"));
            pad.transition.insert('<', map);
        }
        // v
        {
            let mut map = HashMap::new();
            map.insert('^', String::from("^A"));
            map.insert('A', String::from(">^A"));
            map.insert('<', String::from("<A"));
            map.insert('v', String::from("A"));
            map.insert('>', String::from(">A"));
            pad.transition.insert('v', map);
        }
        // >
        {
            let mut map = HashMap::new();
            map.insert('^', String::from("<^A"));
            map.insert('A', String::from("^A"));
            map.insert('<', String::from("<<A"));
            map.insert('v', String::from("<A"));
            map.insert('>', String::from("A"));
            pad.transition.insert('>', map);
        }

        return pad;
    }

    fn transition(&self, old: char, new: char) -> &str {
        self.transition.get(&old).unwrap().get(&new).unwrap()
    }
}

struct NumPad {
    transition: HashMap<char, HashMap<char, String>>,
}

impl NumPad {
    // NOTE: this is more than retarded, one would say this is beyond god
    fn new() -> NumPad {
        let mut pad = NumPad {
            transition: HashMap::new(),
        };

        // A
        {
            let mut map = HashMap::new();
            map.insert('A', String::from("A"));
            map.insert('0', String::from("<A"));
            map.insert('1', String::from("^<<A"));
            map.insert('2', String::from("<^A"));
            map.insert('3', String::from("^A"));
            map.insert('4', String::from("^^<<A"));
            map.insert('5', String::from("^^<A"));
            map.insert('6', String::from("^^A"));
            map.insert('7', String::from("^^^<<A"));
            map.insert('8', String::from("^^^<A"));
            map.insert('9', String::from("^^^A"));
            pad.transition.insert('A', map);
        }

        // 0
        {
            let mut map = HashMap::new();
            map.insert('A', String::from(">A"));
            map.insert('0', String::from("A"));
            map.insert('1', String::from("^<A"));
            map.insert('2', String::from("^A"));
            map.insert('3', String::from("^>A"));
            map.insert('4', String::from("^^<A"));
            map.insert('5', String::from("^^A"));
            map.insert('6', String::from("^^>A"));
            map.insert('7', String::from("^^^<A"));
            map.insert('8', String::from("^^^A"));
            map.insert('9', String::from("^^^>A"));
            pad.transition.insert('0', map);
        }

        // 1
        {
            let mut map = HashMap::new();
            map.insert('A', String::from(">>vA"));
            map.insert('0', String::from(">vA"));
            map.insert('1', String::from("A"));
            map.insert('2', String::from(">A"));
            map.insert('3', String::from(">>A"));
            map.insert('4', String::from("^A"));
            map.insert('5', String::from("^>A"));
            map.insert('6', String::from("^>>A"));
            map.insert('7', String::from("^^A"));
            map.insert('8', String::from("^^>A"));
            map.insert('9', String::from("^^>>A"));
            pad.transition.insert('1', map);
        }

        // 2
        {
            let mut map = HashMap::new();
            map.insert('A', String::from(">vA"));
            map.insert('0', String::from("vA"));
            map.insert('1', String::from("<A"));
            map.insert('2', String::from("A"));
            map.insert('3', String::from(">A"));
            map.insert('4', String::from("^<A"));
            map.insert('5', String::from("^A"));
            map.insert('6', String::from("^>A"));
            map.insert('7', String::from("^^<A"));
            map.insert('8', String::from("^^A"));
            map.insert('9', String::from("^^>A"));
            pad.transition.insert('2', map);
        }

        // 3
        {
            let mut map = HashMap::new();
            map.insert('A', String::from("vA"));
            map.insert('0', String::from("v<A"));
            map.insert('1', String::from("<<A"));
            map.insert('2', String::from("<A"));
            map.insert('3', String::from("A"));
            map.insert('4', String::from("^<<A"));
            map.insert('5', String::from("^<A"));
            map.insert('6', String::from("^A"));
            map.insert('7', String::from("<<^^A"));
            map.insert('8', String::from("^^<A"));
            map.insert('9', String::from("^^A"));
            pad.transition.insert('3', map);
        }

        // 4
        {
            let mut map = HashMap::new();
            map.insert('A', String::from(">>vvA"));
            map.insert('0', String::from(">vvA"));
            map.insert('1', String::from("vA"));
            map.insert('2', String::from("v>A"));
            map.insert('3', String::from("v>>A"));
            map.insert('4', String::from("A"));
            map.insert('5', String::from(">A"));
            map.insert('6', String::from(">>A"));
            map.insert('7', String::from("^A"));
            map.insert('8', String::from("^>A"));
            map.insert('9', String::from("^>>A"));
            pad.transition.insert('4', map);
        }

        // 5
        {
            let mut map = HashMap::new();
            map.insert('A', String::from("vv>A"));
            map.insert('0', String::from("vvA"));
            map.insert('1', String::from("v<A"));
            map.insert('2', String::from("vA"));
            map.insert('3', String::from("v>A"));
            map.insert('4', String::from("<A"));
            map.insert('5', String::from("A"));
            map.insert('6', String::from(">A"));
            map.insert('7', String::from("^<A"));
            map.insert('8', String::from("^A"));
            map.insert('9', String::from("^>A"));
            pad.transition.insert('5', map);
        }

        // 6
        {
            let mut map = HashMap::new();
            map.insert('A', String::from("vvA"));
            map.insert('0', String::from("vv<A"));
            map.insert('1', String::from("v<<A"));
            map.insert('2', String::from("v<A"));
            map.insert('3', String::from("vA"));
            map.insert('4', String::from("<<A"));
            map.insert('5', String::from("<A"));
            map.insert('6', String::from("A"));
            map.insert('7', String::from("^<<A"));
            map.insert('8', String::from("^<A"));
            map.insert('9', String::from("^A"));
            pad.transition.insert('6', map);
        }

        // 7
        {
            let mut map = HashMap::new();
            map.insert('A', String::from(">>vvvA"));
            map.insert('0', String::from(">vvvA"));
            map.insert('1', String::from("vvA"));
            map.insert('2', String::from("vv>A"));
            map.insert('3', String::from("vv>>A"));
            map.insert('4', String::from("vA"));
            map.insert('5', String::from("v>A"));
            map.insert('6', String::from("v>>A"));
            map.insert('7', String::from("A"));
            map.insert('8', String::from(">A"));
            map.insert('9', String::from(">>A"));
            pad.transition.insert('7', map);
        }

        // 8
        {
            let mut map = HashMap::new();
            map.insert('A', String::from("vvv>A"));
            map.insert('0', String::from("vvvA"));
            map.insert('1', String::from("vv<A"));
            map.insert('2', String::from("vvA"));
            map.insert('3', String::from("vv>A"));
            map.insert('4', String::from("v<A"));
            map.insert('5', String::from("vA"));
            map.insert('6', String::from("v>A"));
            map.insert('7', String::from("<A"));
            map.insert('8', String::from("A"));
            map.insert('9', String::from(">A"));
            pad.transition.insert('8', map);
        }

        // 9
        {
            let mut map = HashMap::new();
            map.insert('A', String::from("vvvA"));
            map.insert('0', String::from("vvv<A"));
            map.insert('1', String::from("vv<<A"));
            map.insert('2', String::from("vv<A"));
            map.insert('3', String::from("vvA"));
            map.insert('4', String::from("v<<A"));
            map.insert('5', String::from("v<A"));
            map.insert('6', String::from("vA"));
            map.insert('7', String::from("<<A"));
            map.insert('8', String::from("<A"));
            map.insert('9', String::from("A"));
            pad.transition.insert('9', map);
        }

        return pad;
    }

    fn transition(&self, old: char, new: char) -> &str {
        self.transition.get(&old).unwrap().get(&new).unwrap()
    }

    fn encode(&self, code: &str) -> String {
        let mut old = 'A';
        let mut result = String::new();
        for new in code.chars() {
            result.push_str(self.transition(old, new));
            old = new;
        }

        return result;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Copy, Clone, PartialEq, Eq)]
    struct Point {
        x: isize,
        y: isize,
    }

    impl Point {
        fn new(x: isize, y: isize) -> Point {
            Point { x, y }
        }
        fn to_string(&self) -> String {
            format!("({},{})", self.x, self.y)
        }
    }

    impl std::ops::Add for Point {
        type Output = Point;
        fn add(self, other: Point) -> Point {
            Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    impl std::ops::Sub for Point {
        type Output = Point;
        fn sub(self, other: Point) -> Point {
            Point {
                x: self.x - other.x,
                y: self.y - other.y,
            }
        }
    }

    impl From<char> for Point {
        fn from(c: char) -> Self {
            match c {
                '<' => Point::new(-1, 0),
                '>' => Point::new(1, 0),
                '^' => Point::new(0, -1),
                'v' => Point::new(0, 1),
                'A' => Point::new(0, 0),
                _ => panic!("unexpected instruction"),
            }
        }
    }

    #[test]
    fn test_dirpad_encoding() {
        let num_pad = NumPad::new();
        let dir_pad = DirPad::new();

        let encoding = num_pad.encode("029A");
        let encoding = dir_pad.encode(&encoding);
        assert!(encoding.len() == "v<<A>>^A<A>AvA<^AA>A<vAAA>^A".len());
        let encoding = dir_pad.encode(&encoding);
        assert!(
            encoding.len()
                == "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A".len()
        );
    }

    #[test]
    fn test_dirpad() {
        let mut map = HashMap::new();
        map.insert('^', Point::new(1, 0));
        map.insert('A', Point::new(2, 0));
        map.insert('<', Point::new(0, 1));
        map.insert('v', Point::new(1, 1));
        map.insert('>', Point::new(2, 1));

        let pad = DirPad::new();

        for old in "^A<v>".chars() {
            for new in "^A<v>".chars() {
                let transition = pad.transition(old, new);
                let expected = *map.get(&new).unwrap();
                let mut got = *map.get(&old).unwrap();

                let delta = expected - got;
                assert!(delta.x.abs() + delta.y.abs() + 1 == transition.len().try_into().unwrap());

                for p in transition.chars() {
                    let p = Point::from(p);
                    got = got + p;
                }

                if got != expected {
                    println!("Failure:");
                    println!("\told:\t\t'{old}'");
                    println!("\tnew:\t\t'{new}'");
                    println!("\ttransition:\t'{transition}'");
                    println!("\tgot:\t\t'{}'", got.to_string());
                    println!("\texpected:\t'{}'", expected.to_string());
                    assert!(false);
                }
            }
        }
    }

    #[test]
    fn test_numpad_encode() {
        let pad = NumPad::new();
        let possible = ["<A^A>^^AvvvA", "<A^A^>^AvvvA", "<A^A^^>AvvvA"];
        let encoding = pad.encode("029A");
        let c = possible.iter().filter(|p| *p == &encoding).count();
        assert!(c == 1);
    }

    #[test]
    fn test_pipeline_029a() {
        let numpad = NumPad::new();
        let dirpad = DirPad::new();

        let input = "029A";
        let encoding = numpad.encode(input);
        let encoding = dirpad.encode(&encoding);
        let encoding = dirpad.encode(&encoding);

        assert!(
            encoding.len()
                == "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A".len()
        );
    }

    #[test]
    fn test_pipeline_980a() {
        let numpad = NumPad::new();
        let dirpad = DirPad::new();

        let input = "980A";
        let encoding = numpad.encode(input);
        let encoding = dirpad.encode(&encoding);
        let encoding = dirpad.encode(&encoding);

        assert!(
            encoding.len() == "<v<A>>^AAAvA^A<vA<AA>>^AvAA<^A>A<v<A>A>^AAAvA<^A>A<vA>^A<A>A".len()
        );
    }

    #[test]
    fn test_pipeline_179a() {
        let numpad = NumPad::new();
        let dirpad = DirPad::new();

        let input = "179A";
        let encoding = numpad.encode(input);
        let encoding = dirpad.encode(&encoding);
        let encoding = dirpad.encode(&encoding);

        assert!(
            encoding.len()
                == "<v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A".len()
        );
    }

    #[test]
    fn test_pipeline_456a() {
        let numpad = NumPad::new();
        let dirpad = DirPad::new();

        let input = "456A";
        let encoding = numpad.encode(input);
        let encoding = dirpad.encode(&encoding);
        let encoding = dirpad.encode(&encoding);
        let target = "<v<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>A<v<A>A>^AAvA<^A>A";
        if target.len() != encoding.len() {
            println!("target:   '{target}'");
            println!("encoding: '{encoding}'");

            println!("target:   '{}'", target.len());
            println!("encoding: '{}'", encoding.len());
        }

        assert!(encoding.len() == target.len());
    }

    #[test]
    fn test_pipeline_379a() {
        let numpad = NumPad::new();
        let dirpad = DirPad::new();

        let input = "379A";
        let encoding = numpad.encode(input);
        let encoding = dirpad.encode(&encoding);
        let encoding = dirpad.encode(&encoding);
        let target = "<v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A";
        if target.len() != encoding.len() {
            println!("target:   '{target}'");
            println!("encoding: '{encoding}'");

            println!("target:   '{}'", target.len());
            println!("encoding: '{}'", encoding.len());
        }

        assert!(encoding.len() == target.len());
    }

    #[test]
    fn test_numpad() {
        let mut map = HashMap::new();
        map.insert('7', Point::new(0, 0));
        map.insert('8', Point::new(1, 0));
        map.insert('9', Point::new(2, 0));

        map.insert('4', Point::new(0, 1));
        map.insert('5', Point::new(1, 1));
        map.insert('6', Point::new(2, 1));

        map.insert('1', Point::new(0, 2));
        map.insert('2', Point::new(1, 2));
        map.insert('3', Point::new(2, 2));

        map.insert('0', Point::new(1, 3));
        map.insert('A', Point::new(2, 3));

        let pad = NumPad::new();

        for old in "0123456789A".chars() {
            for new in "0123456789A".chars() {
                let transition = pad.transition(old, new);
                let expected = *map.get(&new).unwrap();
                let mut got = *map.get(&old).unwrap();

                let delta = expected - got;
                assert!(delta.x.abs() + delta.y.abs() + 1 == transition.len().try_into().unwrap());

                for p in transition.chars() {
                    let p = Point::from(p);
                    got = got + p;
                }

                if got != expected {
                    println!("Failure:");
                    println!("\told:\t\t'{old}'");
                    println!("\tnew:\t\t'{new}'");
                    println!("\ttransition:\t'{transition}'");
                    println!("\tgot:\t\t'{}'", got.to_string());
                    println!("\texpected:\t'{}'", expected.to_string());
                    assert!(false);
                }
            }
        }
    }

    #[test]
    fn test_code_complexity() {
        let input = "
029A
980A
179A
456A
379A";

        let num_pad = NumPad::new();
        let dir_pad = DirPad::new();

        let mut sum = 0;

        for code in input.trim().lines().map(|l| l.trim()) {
            let encoding = num_pad.encode(code);
            let encoding = dir_pad.encode(&encoding);
            let encoding = dir_pad.encode(&encoding);

            let code = &code[..code.len() - 1];
            let code: usize = code.parse().unwrap();
            sum += encoding.len() * code;
        }

        assert!(sum == 126384);
    }

    #[test]
    fn part_one_fail_one() {
        let input = "
279A
286A
508A
463A
246A
";

        let num_pad = NumPad::new();
        let dir_pad = DirPad::new();

        let mut sum = 0;

        for code in input.trim().lines().map(|l| l.trim()) {
            let encoding = num_pad.encode(code);
            let encoding = dir_pad.encode(&encoding);
            let encoding = dir_pad.encode(&encoding);

            let code = &code[..code.len() - 1];
            let code: usize = code.parse().unwrap();
            sum += encoding.len() * code;
        }

        assert!(sum < 129874);
    }
}

fn main() {
    let input = "
279A
286A
508A
463A
246A
";

    let num_pad = NumPad::new();
    let dir_pad = DirPad::new();

    let mut sum = 0;

    for code in input.trim().lines().map(|l| l.trim()) {
        let encoding = num_pad.encode(code);
        let encoding = dir_pad.encode(&encoding);
        let encoding = dir_pad.encode(&encoding);

        let code = &code[..code.len() - 1];
        let code: usize = code.parse().unwrap();
        sum += encoding.len() * code;
        println!("{code}: {encoding}");
    }

    println!("Solution 1: {sum}");
}
