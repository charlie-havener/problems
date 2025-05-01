use std::collections::{HashSet, VecDeque};

enum Movements {
    Up,
    Down,
    Left,
    Right,
}

impl Movements {
    // in the form (row_offset, col_offset)
    fn get_offset(&self) -> (i32, i32) {
        match self {
            Self::Up => (-1,0),
            Self::Down => (1,0),
            Self::Left => (0, -1),
            Self::Right => (0, 1),
        }
    }
}

struct SnakeGame {
    snake_order: VecDeque<(i32, i32)>,
    snake_pos: HashSet<(i32, i32)>,
    food: Vec<Vec<i32>>,
    food_ptr: usize,
    width: i32,
    height: i32,
}

impl SnakeGame {
    
    fn new(width: i32, height: i32, food: Vec<Vec<i32>>) -> Self {
        Self {
            snake_order: VecDeque::from([(0,0)]),
            snake_pos: HashSet::from([(0,0)]),
            food,
            food_ptr: 0,
            width,
            height,
        }
    }

    fn make_a_move(&mut self, direction: String) -> i32 {
        let m = match direction.as_str() {
            "U" => Movements::Up,
            "D" => Movements::Down,
            "L" => Movements::Left,
            "R" => Movements::Right,
            _ => panic!("naughty"),
        }.get_offset();

        // the snake will always be at least length 1 so unwrap is safe
        let current_pos = self.snake_order.back().unwrap();
        let next_pos = (current_pos.0 as i32 + m.0, current_pos.1 as i32 + m.1);
        println!("{next_pos:?}");

        // if the move is out of bounds then dead
        if next_pos.0 < 0 || next_pos.0 >= self.height || next_pos.1 < 0 || next_pos.1 >= self.width {
            return -1;
        }

        // if the move is onto food, then grow and occupy the space
        if self.food_ptr < self.food.len() && next_pos.0 == self.food[self.food_ptr][0] as i32 && next_pos.1 == self.food[self.food_ptr][1] as i32 {
            self.snake_order.push_back(next_pos);
            self.snake_pos.insert(next_pos);
            self.food_ptr += 1;
            return self.snake_order.len() as i32 - 1;
        }

        // otherwise it's just a move so remove the tail then move the head
        // if the head is onto it's body then dead
        let p = self.snake_order.pop_front().unwrap();
        self.snake_pos.remove(&p);
        if self.snake_pos.contains(&next_pos) {
            return -1;
        }
        self.snake_order.push_back(next_pos);
        self.snake_pos.insert(next_pos);
        return self.snake_pos.len() as i32 -1;
    }

}
