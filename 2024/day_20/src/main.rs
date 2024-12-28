use std::collections::HashMap;

fn solve(
    map: &Vec<Vec<char>>,
    seen: &mut HashMap<(isize, isize), usize>,
    x: isize,
    y: isize,
    cost: usize,
) -> Option<usize> {
    match seen.get(&(x, y)) {
        Some(val) if *val < cost => return None,
        _ => (),
    }
    seen.insert((x, y), cost);

    if x < 0 || y < 0 {
        return None;
    }
    let x = x as usize;
    let y = y as usize;

    if y >= map.len() || x >= map[y].len() {
        return None;
    }

    match map[y][x] {
        'E' => return Some(cost),
        '#' => return None,
        _ => (),
    }
    let mut child = [None, None, None, None];

    let x = x as isize;
    let y = y as isize;

    child[0] = solve(map, seen, x + 1, y, cost + 1);
    child[1] = solve(map, seen, x - 1, y, cost + 1);
    child[2] = solve(map, seen, x, y + 1, cost + 1);
    child[3] = solve(map, seen, x, y - 1, cost + 1);

    child.iter().fold(None, |acc, e| match (acc, *e) {
        (None, None) => None,
        (Some(val), None) => Some(val),
        (None, Some(val)) => Some(val),
        (Some(a), Some(b)) => Some(if a < b { a } else { b }),
    })
}

fn main() {
    let input = "
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";
    let mut map: Vec<Vec<char>> = input
        .trim()
        .lines()
        .map(|l| l.trim().chars().collect())
        .collect();

    let mut start_x = 0;
    let mut start_y = 0;
    'outer: for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == 'S' {
                map[y][x] = '.';
                start_x = x as isize;
                start_y = y as isize;
                break 'outer;
            }
        }
    }

    let mut seen = HashMap::new();
    let base_time = solve(&map, &mut seen, start_x, start_y, 0).unwrap();
    println!("Base Time: {base_time}");

    let solutions = Vec::new();

    for y in 1..map.len() - 1 {
        for x in 1..map[y].len() - 1 {
            let x = x as isize;
            let y = y as isize;

            for dy in -1..=1 {
                for dx in -1..=1 {
                }
            }

        }
    }

}
