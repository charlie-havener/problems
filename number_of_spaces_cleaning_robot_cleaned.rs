enum CWDirections {
    Right,
    Down,
    Left,
    Up,
}

impl CWDirections {
    fn rotate(&mut self) {
        match self {
            Self::Right => *self = Self::Down,
            Self::Down => *self = Self::Left,
            Self::Left => *self = Self::Up,
            Self::Up => *self = Self::Right,
        }
    }

    fn get_dir_bit(&self) -> i32 {
        match self {
            Self::Right => 1,
            Self::Down => 2,
            Self::Left => 3,
            Self::Up => 4,
        }
    }
}

fn get_next_pos_coords(curr_pos: (i32, i32), dir: &CWDirections) -> (i32, i32) {
    let (r,c) = curr_pos;
    return match dir {
        CWDirections::Right => (r, c+1),
        CWDirections::Down => (r+1, c),
        CWDirections::Left => (r, c-1),
        CWDirections::Up => (r-1, c),
    };
}

fn is_valid_pos(pos: (i32, i32), room: &Vec<Vec<i32>>) -> bool {
    // valid positions are within the room and are not objects (1s)
    let (r, c) = pos;
    return r >= 0 && c >= 0 
           && r + 1 <= room.len() as i32 && c + 1 <= room[0].len() as i32
           && room[r as usize][c as usize] != 1;
}


pub fn number_of_clean_rooms(mut room: Vec<Vec<i32>>) -> i32 {
    let mut ans = 0;
    let mut dir = CWDirections::Right;
    let mut pos: (i32, i32) = (0,0);

    loop {
        let (r,c) = (pos.0 as usize, pos.1 as usize);

        // unvisited rooms are 0s
        if room[r][c] == 0 { ans += 1 }

        // mark the room as visited with dir and return if we have been in this scenaria before
        let t = room[r][c];
        room[r][c] |= 1 << dir.get_dir_bit();
        if room[r][c] == t { return ans; }


        let next_pos = get_next_pos_coords(pos, &dir);
        if is_valid_pos(next_pos, &room) {
            pos = next_pos;
        } else {
            dir.rotate();
        }
    }
}

#[test]
fn tests() {
    
    let room = vec![vec![0,0,0],vec![1,1,0],vec![0,0,0]];
    assert_eq!(7, number_of_clean_rooms(room));

    let room = vec![vec![0,1,0],vec![1,0,0],vec![0,0,0]];
    assert_eq!(1, number_of_clean_rooms(room));

    let room = vec![vec![0,0,0],vec![0,0,0],vec![0,0,0]];
    assert_eq!(8, number_of_clean_rooms(room));
}
