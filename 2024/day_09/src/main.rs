mod problem_1 {
    #[derive(Copy, Clone)]
    enum Block {
        Empty,
        Full(usize),
    }

    pub struct Problem1<'a> {
        input: &'a str,
    }

    fn is_even(n: usize) -> bool {
        (n & 1) == 0
    }

    impl<'a> Problem1<'a> {
        pub fn new(input: &'a str) -> Problem1<'a> {
            Problem1 { input }
        }

        pub fn solve(&self) {
            let mut disk = vec![];
            for i in 0..self.input.trim().len() {
                let b = if is_even(i) {
                    Block::Full(i >> 1)
                } else {
                    Block::Empty
                };
                let n: usize = self.input[i..i + 1].parse().unwrap();
                for _ in 0..n {
                    disk.push(b);
                }
            }

            let mut front = Self::next_empty(&disk, 0);
            let mut back = Self::previous_full(&disk, disk.len() - 1);
            while back > front {
                disk[front] = disk[back];
                disk[back] = Block::Empty;
                front = Self::next_empty(&disk, front);
                back = Self::previous_full(&disk, back);
            }

            let mut checksum = 0;
            for (i, b) in disk.iter().enumerate() {
                match b {
                    Block::Empty => continue,
                    Block::Full(id) => checksum += id * i,
                }
            }
            println!("Solution 1: {}", checksum);
        }

        fn next_empty(disk: &Vec<Block>, index: usize) -> usize {
            for i in index..disk.len() {
                match disk[i] {
                    Block::Empty => return i,
                    Block::Full(_) => continue,
                }
            }
            disk.len()
        }

        fn previous_full(disk: &Vec<Block>, index: usize) -> usize {
            for i in (0..=index).rev() {
                match disk[i] {
                    Block::Empty => continue,
                    Block::Full(_) => return i,
                }
            }
            0
        }
    }
}

mod problem_2 {

    #[derive(Copy, Clone)]
    enum Block {
        Empty,
        Full(usize),
    }

    pub fn solve(input: &str) {
        let input = input.trim();

        let mut disk = vec![];

        for i in 0..input.len() {
            let n: usize = input[i..i + 1].parse().unwrap();
            let b = if is_even(i) {
                Block::Full(i >> 1)
            } else {
                Block::Empty
            };
            for _ in 0..n {
                disk.push(b);
            }
        }

        let mut back = disk.len();
        loop {
            let back_len: usize;
            (back, back_len) = if let Some(v) = prev_full(&disk, back) {
                v
            } else {
                break;
            };

            let mut front = 0;
            loop {
                let front_len: usize;
                (front, front_len) = if let Some(v) = next_empty(&disk, front) {
                    v
                } else {
                    break;
                };

                if front_len >= back_len {
                    break;
                }
            }

            if front >= back {
                continue;
            }
            for i in 0..back_len {
                disk[front + i] = disk[back + i];
                disk[back + i] = Block::Empty;
            }
        }

        let mut checksum = 0;
        for (i, b) in disk.iter().enumerate() {
            match b {
                Block::Empty => continue,
                Block::Full(id) => checksum += id * i,
            }
        }
        println!("Solution 2: {}", checksum);
    }

    #[allow(dead_code)]
    fn disk_print(disk: &Vec<Block>) {
        for b in disk.iter() {
            match b {
                Block::Empty => print!("."),
                Block::Full(id) => print!("{id}"),
            }
        }
        println!();
    }

    fn next_empty(disk: &Vec<Block>, index: usize) -> Option<(usize, usize)> {
        let mut i = index + 1;
        while i < disk.len() {
            match disk[i] {
                Block::Empty => break,
                Block::Full(_) => (),
            }
            i += 1;
        }
        if i == disk.len() {
            return None;
        }
        let mut j = i;

        while j < disk.len() {
            match disk[j] {
                Block::Empty => j += 1,
                Block::Full(_) => break,
            }
        }
        Some((i, j - i))
    }

    fn find_prev_full(disk: &Vec<Block>, index: usize) -> Option<usize> {
        for (j, b) in (&disk[..index]).iter().enumerate().rev() {
            match b {
                Block::Empty => continue,
                Block::Full(_) => return Some(j),
            }
        }
        None
    }

    ///
    fn find_start_of_prev_full(disk: &Vec<Block>, index: usize) -> usize {
        let id = match disk[index] {
            Block::Full(id) => id,
            _ => unreachable!(),
        };

        let mut index = index;

        for (i, b) in disk[..index + 1].iter().enumerate().rev() {
            match b {
                Block::Full(n) if id == *n => index = i,
                _ => break,
            }
        }

        index
    }

    fn prev_full(disk: &Vec<Block>, index: usize) -> Option<(usize, usize)> {
        let j = find_prev_full(disk, index)?;
        let i = find_start_of_prev_full(disk, j);
        Some((i, j + 1 - i))
    }

    fn is_even(i: usize) -> bool {
        (i & 1) == 0
    }
}

fn main() {
    //let input = "2333133121414131402";
    let input = &std::fs::read_to_string("input").unwrap();
    problem_1::Problem1::new(input).solve();
    problem_2::solve(input);
}
