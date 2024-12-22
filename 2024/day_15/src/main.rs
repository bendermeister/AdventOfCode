mod part_1 {

    #[derive(Copy, Clone)]
    enum Tile {
        Wall,
        Free,
        Box,
        Robot,
    }

    impl std::str::FromStr for Tile {
        type Err = ();
        fn from_str(text: &str) -> Result<Tile, Self::Err> {
            if text.len() != 1 {
                return Err(());
            }
            match text {
                "." => Ok(Tile::Free),
                "#" => Ok(Tile::Wall),
                "@" => Ok(Tile::Robot),
                "O" => Ok(Tile::Box),
                _ => Err(()),
            }
        }
    }

    #[derive(Copy, Clone)]
    enum Command {
        Up,
        Down,
        Left,
        Right,
    }

    impl std::str::FromStr for Command {
        type Err = ();
        fn from_str(text: &str) -> Result<Command, Self::Err> {
            if text.len() != 1 {
                return Err(());
            }
            match text {
                "<" => Ok(Command::Left),
                ">" => Ok(Command::Right),
                "^" => Ok(Command::Up),
                "v" => Ok(Command::Down),
                _ => Err(()),
            }
        }
    }

    impl Command {
        fn to_point(&self) -> Point {
            match self {
                Command::Down => Point::new(0, 1),
                Command::Up => Point::new(0, -1),
                Command::Left => Point::new(-1, 0),
                Command::Right => Point::new(1, 0),
            }
        }
    }

    #[derive(Copy, Clone)]
    struct Point {
        x: isize,
        y: isize,
    }

    impl Point {
        fn to_coords(self) -> (usize, usize) {
            (self.x as usize, self.y as usize)
        }
        fn new(x: isize, y: isize) -> Point {
            Point { x, y }
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

    fn move_in_map(me: Point, cmd: Command, map: &mut Vec<Vec<Tile>>) -> Option<()> {
        let (x, y) = me.to_coords();
        let my_tile = map[y][x];
        let next_pos = me + cmd.to_point();
        let (next_x, next_y) = next_pos.to_coords();

        match map[y][x].clone() {
            Tile::Robot | Tile::Box => match move_in_map(next_pos, cmd, map) {
                Some(_) => {
                    map[y][x] = Tile::Free;
                    map[next_y][next_x] = my_tile;
                    Some(())
                }
                None => None,
            },
            Tile::Free => Some(()),
            Tile::Wall => None,
        }
    }

    pub fn solve(map: &str, commands: &str) {
        // Parse map
        let mut map: Vec<Vec<Tile>> = map
            .trim()
            .lines()
            .map(|l| l.trim())
            .map(|l| (0..l.len()).map(|i| l[i..i + 1].parse().unwrap()).collect())
            .collect();

        // Parse Commands
        let commands: Vec<Command> = (0..commands.len())
            .map(|i| commands[i..i + 1].parse())
            .filter(|r| match r {
                Ok(_) => true,
                Err(_) => false,
            })
            .map(|r| r.unwrap())
            .collect();

        // Find Robot on Map
        let mut robot = Point::new(0, 0);
        for y in 0..map.len() {
            for x in 0..map[y].len() {
                match map[y][x] {
                    Tile::Robot => robot = Point::new(x as isize, y as isize),
                    _ => continue,
                }
            }
        }

        for cmd in commands {
            match move_in_map(robot, cmd, &mut map) {
                Some(_) => robot = robot + cmd.to_point(),
                None => (),
            }
        }

        let mut sum = 0;
        for y in 0..map.len() {
            for x in 0..map[y].len() {
                if let Tile::Box = map[y][x] {
                    sum += y * 100 + x;
                }
            }
        }

        println!("Solution 1: {sum}");
    }
}

mod part_2 {

    #[derive(Clone, Copy, PartialEq, Eq)]
    enum Tile {
        Wall,
        Robot,
        Free,
        BoxOpen,
        BoxClose,
    }

    impl std::str::FromStr for Tile {
        type Err = ();
        fn from_str(text: &str) -> Result<Tile, Self::Err> {
            let tile = match text {
                "." => Tile::Free,
                "[" => Tile::BoxOpen,
                "]" => Tile::BoxClose,
                "@" => Tile::Robot,
                "#" => Tile::Wall,
                _ => return Err(()),
            };
            Ok(tile)
        }
    }

    #[derive(Clone, Copy, PartialEq, Eq)]
    enum Command {
        Up,
        Down,
        Left,
        Right,
    }

    impl std::str::FromStr for Command {
        type Err = ();
        fn from_str(text: &str) -> Result<Command, Self::Err> {
            if text.len() != 1 {
                return Err(());
            }
            match text {
                "<" => Ok(Command::Left),
                ">" => Ok(Command::Right),
                "^" => Ok(Command::Up),
                "v" => Ok(Command::Down),
                _ => Err(()),
            }
        }
    }

    #[derive(Clone, Copy)]
    struct Point {
        x: usize,
        y: usize,
    }

    impl Point {
        fn to_coords(self) -> (usize, usize) {
            (self.x, self.y)
        }
        fn new(x: usize, y: usize) -> Point {
            Point { x, y }
        }
        fn apply_command(&self, cmd: Command) -> Point {
            match cmd {
                Command::Up => Point::new(self.x, self.y - 1),
                Command::Down => Point::new(self.x, self.y + 1),
                Command::Left => Point::new(self.x - 1, self.y),
                Command::Right => Point::new(self.x + 1, self.y),
            }
        }
    }

    fn is_moveable(me: Point, cmd: Command, map: &Vec<Vec<Tile>>) -> bool {
        let (x, y) = me.to_coords();
        match map[y][x] {
            Tile::Free => true,
            Tile::Wall => false,
            Tile::Robot => is_moveable(me.apply_command(cmd), cmd, map),
            Tile::BoxOpen | Tile::BoxClose if cmd == Command::Left || cmd == Command::Right => {
                is_moveable(me.apply_command(cmd), cmd, map)
            }
            Tile::BoxOpen => {
                is_moveable(me.apply_command(cmd), cmd, map)
                    && is_moveable(
                        me.apply_command(Command::Right).apply_command(cmd),
                        cmd,
                        map,
                    )
            }
            Tile::BoxClose => {
                is_moveable(me.apply_command(cmd), cmd, map)
                    && is_moveable(
                        me.apply_command(Command::Left).apply_command(cmd),
                        cmd,
                        map,
                    )
            }
        }
    }

    /// THIS FUNCTION EXPECTS TO BE RUN AFTER `is_moveable` RETURNS TRUE FOR THE SAME ARGUMENTS!
    fn force_move(me: Point, cmd: Command, map: &mut Vec<Vec<Tile>>) {
        let (x, y) = me.to_coords();
        match map[y][x] {
            Tile::Free | Tile::Wall => (),
            Tile::Robot => {
                force_move(me.apply_command(cmd), cmd, map);
                let (nx, ny) = me.apply_command(cmd).to_coords();
                map[ny][nx] = map[y][x];
                map[y][x] = Tile::Free;
            }
            Tile::BoxOpen | Tile::BoxClose if cmd == Command::Left || cmd == Command::Right => {
                force_move(me.apply_command(cmd), cmd, map);
                let (nx, ny) = me.apply_command(cmd).to_coords();
                map[ny][nx] = map[y][x];
                map[y][x] = Tile::Free;
            }
            Tile::BoxOpen => {
                let brother = me.apply_command(Command::Right);

                force_move(me.apply_command(cmd), cmd, map);
                force_move(brother.apply_command(cmd), cmd, map);

                let (bx, by) = brother.to_coords();
                let (nx, ny) = me.apply_command(cmd).to_coords();
                let (bnx, bny) = brother.apply_command(cmd).to_coords();

                map[ny][nx] = map[y][x];
                map[y][x] = Tile::Free;

                map[bny][bnx] = map[by][bx];
                map[by][bx] = Tile::Free;
            }
            Tile::BoxClose => {
                let brother = me.apply_command(Command::Left);

                force_move(me.apply_command(cmd), cmd, map);
                force_move(brother.apply_command(cmd), cmd, map);

                let (bx, by) = brother.to_coords();
                let (nx, ny) = me.apply_command(cmd).to_coords();
                let (bnx, bny) = brother.apply_command(cmd).to_coords();

                map[ny][nx] = map[y][x];
                map[y][x] = Tile::Free;

                map[bny][bnx] = map[by][bx];
                map[by][bx] = Tile::Free;
            }
        }
    }

    pub fn solve(map: &str, commands: &str) {
        let map: String = map
            .trim()
            .lines()
            .map(|l| l.trim())
            .map(|l| {
                l.chars()
                    .map(|c| match c {
                        '#' => "##",
                        '.' => "..",
                        'O' => "[]",
                        '@' => "@.",
                        _ => "",
                    })
                    .collect::<String>()
            })
            .map(|l| l + "\n")
            .collect();

        // NOTE: this iterator trick does not work for none ASCII characters
        let mut map: Vec<Vec<Tile>> = map
            .trim()
            .lines()
            .map(|l| l.trim())
            .map(|l| (0..l.len()).map(|i| l[i..i + 1].parse().unwrap()).collect())
            .collect();

        let commands: Vec<Command> = (0..commands.len())
            .map(|i| commands[i..i + 1].parse())
            .filter(|r| match r {
                Ok(_) => true,
                Err(_) => false,
            })
            .map(|r| r.unwrap())
            .collect();

        let mut robot = Point::new(0, 0);
        // find roboter
        'outer: for y in 0..map.len() {
            for x in 0..map[y].len() {
                if let Tile::Robot = map[y][x] {
                    robot = Point::new(x, y);
                    break 'outer;
                }
            }
        }

        for c in commands {
            if is_moveable(robot, c, &map) {
                force_move(robot, c, &mut map);
                let (x, y) = robot.to_coords();
                map[y][x] = Tile::Free;
                robot = robot.apply_command(c);
                let (x, y) = robot.to_coords();
                map[y][x] = Tile::Robot;
            }
        }

        let mut sum = 0;
        for y in 0..map.len() {
            for x in 0..map[y].len() {
                if let Tile::BoxOpen = map[y][x] {
                    sum += y * 100 + x;
                }
            }
        }

        println!("Solution 2: {sum}");
    }
}

fn main() {
    let map = "
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########
";

    let commands = "
<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";

    let map = &std::fs::read_to_string("input_map").unwrap();
    let commands = &std::fs::read_to_string("input_commands").unwrap();
    part_1::solve(map, commands);
    part_2::solve(map, commands);
}
