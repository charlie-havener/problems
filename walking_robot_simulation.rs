enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn cw(&mut self) {
        match self {
            Direction::Up => *self = Direction::Right,
            Direction::Right => *self = Direction::Down,
            Direction::Down => *self = Direction::Left,
            Direction::Left => *self = Direction::Up,
        };
    }
    fn ccw(&mut self) {
        match self {
            Direction::Up => *self = Direction::Left,
            Direction::Right => *self = Direction::Up,
            Direction::Down => *self = Direction::Right,
            Direction::Left => *self = Direction::Down,
        };
    }
    
    fn movement(&self) -> Position {
        match self {
            Direction::Up => Position{x:0,y:1},
            Direction::Right => Position{x:1,y:0},
            Direction::Down => Position{x:0,y:-1},
            Direction::Left => Position{x:-1,y:0},
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, std::hash::Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new() -> Self {
        Self { x:0, y:0 }
    }
    fn dist(&self) -> i32 {
        self.x * self.x + self.y * self.y
    }
}

impl std::ops::Add for Position {
    type Output = Position;
    fn add(self, rhs: Self) -> Self::Output {
        Self { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

struct RobotSim {
    pos: Position,
    dir: Direction,
    obs: HashSet<Position>,
    max_dist: i32,
}

impl RobotSim {

    fn new(obs: Vec<Vec<i32>>) -> Self {
        let obs: HashSet<Position> = obs.iter().fold(HashSet::new(), |mut acc, ob| {
            acc.insert(Position{x:ob[0], y:ob[1]});
            acc
        });

        let p = Position::new();
        Self {
            max_dist: p.dist(),
            pos: p,
            dir: Direction::Up,
            obs,
        }
    }

    fn simulate_command(&mut self, command: i32) {
        match command {
            -1 => self.dir.cw(),
            -2 => self.dir.ccw(),
            k => {
                let mv = self.dir.movement();
                for _ in 0..k {
                    let next_pos = self.pos + mv;
                    if self.obs.contains(&next_pos) {
                        return;
                    }
                    self.pos = next_pos;
                    self.max_dist = self.max_dist.max(self.pos.dist());
                }
            },
        }
    }
}

use std::collections::HashSet;

pub fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
    let mut robot = RobotSim::new(obstacles);
    for command in commands {
        robot.simulate_command(command)
    }
    return robot.max_dist;
}

#[test]
fn tests() {
    let commands = vec![4,-1,3];
    let obstacles = vec![];
    assert_eq!(25, robot_sim(commands, obstacles));

    let commands = vec![4,-1,4,-2,4];
    let obstacles = vec![vec![2,4]];
    assert_eq!(65, robot_sim(commands, obstacles));

    let commands = vec![6,-1,-1,6];
    let obstacles = vec![];
    assert_eq!(36, robot_sim(commands, obstacles));
}
