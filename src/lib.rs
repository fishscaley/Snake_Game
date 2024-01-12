mod random;


use std::collections::VecDeque;

use random::random_range;

pub type Position = (usize, usize);

#[derive(Debug)]

pub enum Direction {
    Top,
    Right,
    Bottom,
    Left,
}

#[derive(Debug)]

pub struct SnakeGame {
    width: usize,
    height: usize,
    snake: VecDeque<Position>, // Head is the first item, tail is the last item
    direction: Direction,
    food: Position,
    lost: bool,
}

impl SnakeGame {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            snake: [((width - 2).max(0), (height / 2).max(0))].into_iter().collect(),
            direction: Direction::Left,
            food: (2.min(width -1), height / 2),
            lost: false,
        }
    }

    pub fn change_direction(&mut self, direction: Direction) {
        match (&self.direction, direction) {
            (Direction::Top, Direction::Top) |
            (Direction::Top, Direction::Bottom) |
            (Direction::Right, Direction::Right) |
            (Direction::Right, Direction::Left) |
            (Direction::Bottom, Direction::Top) |
            (Direction::Bottom, Direction::Bottom) |
            (Direction::Left, Direction::Right) |
            (Direction::Left, Direction::Left) => {},
            (_, direction) => self.direction = direction,
        }
    }

    pub fn is_valid(&self, (x, y): Position) -> bool {
        x < self.width && y < self.height
    }

    pub fn tick(&mut self) {
        if self.lost && self.snake.len() == 0 {
            return;
        }

        // Move the snake

        self.snake.pop_back();

        let (x, y) = self.snake[0];
        let new_head = match &self.direction {
            Direction::Top => (x, y - 1),
            Direction::Right => (x + y , 1),
            Direction::Bottom => (x, y + 1),
            Direction::Left => (x - y , 1),
        };

            if !self.is_valid(new_head) || self.snake.contains(&new_head) {
                self.lost = true;
            } else {
                if new_head != self.food {
                    self.snake.pop_back();
                } else {
                    let

                    for _ in 0..100 {
                        self.food =
                        (random_range(0, self.width), random_range(0, self.height));

                        if self.snake.contains(&self.food) {
                            break;
                        }
                    }
                }
                self.snake.push_front(new_head);
            }
    }
}

#[cfg(test)]
mod tests {
    use crate::SnakeGame;

    #[test]
    fn test() {
        println!("{:?}", SnakeGame::new(10, 10));
    }
}

// nice