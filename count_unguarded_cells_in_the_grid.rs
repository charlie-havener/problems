#[derive(Clone)]
pub enum CellStatus {
    Unguarded,
    Guarded,
    Guard,
    Wall,
}

pub fn count_unguarded(m: i32, n: i32, guards: Vec<Vec<i32>>, walls: Vec<Vec<i32>>) -> i32 {

    let mut cells: Vec<Vec<CellStatus>> = vec![vec![CellStatus::Unguarded; n as usize]; m as usize];
    let mut unguarded_count: i32 = m * n;

    for wall in walls {
        let (r,c) = (wall[0] as usize, wall[1] as usize);
        cells[r][c] = CellStatus::Wall;
        unguarded_count -= 1;
    }

    for guard in &guards {
        let (r,c) = (guard[0] as usize, guard[1] as usize);
        cells[r][c] = CellStatus::Guard;
        unguarded_count -= 1;
    }

    // simulate each guard
    // stop direction if hitting a wall, another guard, or an end
    for guard in &guards {
        let (r,c) = (guard[0] as usize, guard[1] as usize);
        println!("\nGuard :: {r}, {c}");
        
        // up
        for row in (0..r).rev() {
            println!("up: {row},{c}");
            match cells[row][c] {
                CellStatus::Unguarded => {
                    cells[row][c] = CellStatus::Guarded;
                    unguarded_count -= 1;
                },
                CellStatus::Wall | CellStatus::Guard => break,
                CellStatus::Guarded => continue,
            }
        }

        // down
        for row in (r+1)..m as usize {
            println!("down: {row},{c}");
            match cells[row][c] {
                CellStatus::Unguarded => {
                    cells[row][c] = CellStatus::Guarded;
                    unguarded_count -= 1;
                },
                CellStatus::Wall | CellStatus::Guard => break,
                CellStatus::Guarded => continue,
            }
        }

        // left
        for col in (0..c).rev() {
            println!("left: {r},{col}");
            match cells[r][col] {
                CellStatus::Unguarded => {
                    cells[r][col] = CellStatus::Guarded;
                    unguarded_count -= 1;
                },
                CellStatus::Wall | CellStatus::Guard => break,
                CellStatus::Guarded => continue,
            }
        }

        // right
        for col in (c+1)..n as usize {
            println!("right: {r},{col}");
            match cells[r][col] {
                CellStatus::Unguarded => {
                    cells[r][col] = CellStatus::Guarded;
                    unguarded_count -= 1;
                },
                CellStatus::Wall | CellStatus::Guard => break,
                CellStatus::Guarded => continue,
            }
        }
    }

    return unguarded_count;
}

#[test]
fn test() {
    let m = 4;
    let n = 6;
    let guards = vec![vec![0,0],vec![1,1],vec![2,3]];
    let walls = vec![vec![0,1],vec![2,2],vec![1,4]];
    assert_eq!(7, count_unguarded(m, n, guards, walls));
}









