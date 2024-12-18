
mod problem_2 {
    fn find_score(
        map: &Vec<Vec<i8>>,
        visited: &mut Vec<Vec<bool>>,
        x: isize,
        y: isize,
        parent: i8,
    ) -> usize {
        //println!("{} x: {x}, y: {y}, p: {parent}", " ".repeat((parent + 1) as usize));
        if x < 0 || y < 0 {
            return 0;
        }
        let x = x as usize;
        let y = y as usize;

        if y >= map.len() {
            return 0;
        }
        if x >= map[y].len() {
            return 0;
        }

        if visited[y][x] {
            return 0;
        }
        if map[y][x] != parent + 1 {
            return 0;
        }
        if map[y][x] == 9 {
            return 1;
        }
        visited[y][x] = true;

        let x = x as isize;
        let y = y as isize;

        let a = find_score(map, visited, x + 1, y, parent + 1);
        let b = find_score(map, visited, x - 1, y, parent + 1);
        let c = find_score(map, visited, x, y + 1, parent + 1);
        let d = find_score(map, visited, x, y - 1, parent + 1);

        visited[y as usize][x as usize] = false;

        return a + b + c + d;
    }

    fn reset_visited(visited: &mut Vec<Vec<bool>>) {
        for y in 0..visited.len() {
            for x in 0..visited[y].len() {
                visited[y][x] = false;
            }
        }
    }

    pub fn solve(input: &str) {
        let input = input.trim();
        let map: Vec<Vec<i8>> = input
            .lines()
            .map(|l| l.trim())
            .map(|l| {
                l.as_bytes()
                    .into_iter()
                    .map(|n| (*n as i8) - '0' as i8)
                    .collect()
            })
            .collect();
        let mut visited: Vec<Vec<bool>> = map
            .iter()
            .map(|row| row.iter().map(|_| false).collect())
            .collect();

        let mut sum = 0;
        for y in 0..map.len() {
            for x in 0..map[y].len() {
                if map[y][x] != 0 {
                    continue;
                }
                sum += find_score(&map, &mut visited, x as isize, y as isize, -1);
                reset_visited(&mut visited);
            }
        }
        println!("Solution 2: {sum}");
    }
}

mod problem_1 {
    fn find_score(
        map: &Vec<Vec<i8>>,
        visited: &mut Vec<Vec<bool>>,
        x: isize,
        y: isize,
        parent: i8,
    ) -> usize {
        //println!("{} x: {x}, y: {y}, p: {parent}", " ".repeat((parent + 1) as usize));
        if x < 0 || y < 0 {
            return 0;
        }
        let x = x as usize;
        let y = y as usize;

        if y >= map.len() {
            return 0;
        }
        if x >= map[y].len() {
            return 0;
        }

        if visited[y][x] {
            return 0;
        }
        if map[y][x] != parent + 1 {
            return 0;
        }
        visited[y][x] = true;
        if map[y][x] == 9 {
            return 1;
        }

        let x = x as isize;
        let y = y as isize;

        let a = find_score(map, visited, x + 1, y, parent + 1);
        let b = find_score(map, visited, x - 1, y, parent + 1);
        let c = find_score(map, visited, x, y + 1, parent + 1);
        let d = find_score(map, visited, x, y - 1, parent + 1);

        visited[y as usize][x as usize] = false;

        return a + b + c + d;
    }

    fn reset_visited(visited: &mut Vec<Vec<bool>>) {
        for y in 0..visited.len() {
            for x in 0..visited[y].len() {
                visited[y][x] = false;
            }
        }
    }

    pub fn solve(input: &str) {
        let input = input.trim();
        let map: Vec<Vec<i8>> = input
            .lines()
            .map(|l| l.trim())
            .map(|l| {
                l.as_bytes()
                    .into_iter()
                    .map(|n| (*n as i8) - '0' as i8)
                    .collect()
            })
            .collect();
        let mut visited: Vec<Vec<bool>> = map
            .iter()
            .map(|row| row.iter().map(|_| false).collect())
            .collect();

        let mut sum = 0;
        for y in 0..map.len() {
            for x in 0..map[y].len() {
                if map[y][x] != 0 {
                    continue;
                }
                sum += find_score(&map, &mut visited, x as isize, y as isize, -1);
                reset_visited(&mut visited);
            }
        }
        println!("Solution 1: {sum}");
    }
}

fn main() {
    let input = "
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";
    let input = &std::fs::read_to_string("input").unwrap();
    problem_1::solve(input);
    problem_2::solve(input);
}
