#[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct Robot {
    starting_pos: i32,
    health: i32,
    direction: char,
    initial_idx: usize,
}


pub fn survived_robots_healths(positions: Vec<i32>, health: Vec<i32>, directions: String) -> Vec<i32> {
    
    let mut c = directions.chars();
    let mut robots: Vec<Robot> = (0..positions.len()).fold(Vec::new(), |mut acc, i| {
        acc.push(
            Robot{
                starting_pos: positions[i],
                health: health[i],
                direction: c.next().unwrap(),
                initial_idx: i 
            }
        );
        acc
    });
    robots.sort();

    let collide = |mut a: Robot, mut b: Robot| -> Option<Robot> {
        if a.health == b.health { return None }
        else if a.health > b.health {
            a.health -= 1;
            return Some(a);
        }
        else {
            b.health -= 1;
            return Some(b);
        }

    };

    let mut stack: Vec<Robot> = Vec::new();
    for mut r in robots {
        loop {
            let collision = r.direction == 'L' && !stack.is_empty() && stack.last().unwrap().direction == 'R';
            if !collision {
                stack.push(r);
                break;
            }
            else {
                match collide(r, stack.pop().unwrap()) {
                    Some(winner) => {
                        r = winner;
                    }
                    None => break,
                }
            }
        }
    }

    stack.sort_by_key(|r| r.initial_idx);
    return stack.into_iter().map(|r| r.health).collect();
}

#[test]
fn tests() {
    let p =  vec![5,4,3,2,1];
    let h = vec![2,17,9,15,10];
    let d = String::from("RRRRR");
    assert_eq!(vec![2,17,9,15,10], survived_robots_healths(p,h,d));

    let p =  vec![3,5,2,6];
    let h = vec![10,10,15,12];
    let d = String::from("RLRL");
    assert_eq!(vec![14], survived_robots_healths(p,h,d));

    let p =  vec![1,2,5,6];
    let h = vec![10,10,11,11];
    let d = String::from("RLRL");
    assert_eq!(Vec::<i32>::new(), survived_robots_healths(p,h,d));

    let p =  vec![11,44,16];
    let h = vec![1,20,17];
    let d = String::from("RLR");
    assert_eq!(vec![18], survived_robots_healths(p,h,d));
}
