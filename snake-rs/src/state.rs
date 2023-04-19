use bevy::prelude::Component;

use arcade_util::{Coord2D, CoordConfiguration, GameStatus, Collidable};
use crate::snake::Snake;
use crate::board::Board;

#[derive(Component)]
pub struct SnakeState {
    snake: Snake,
    board: Board,
}

impl Default for SnakeState {
    fn default() -> Self {
        SnakeState::new(10, 10)
    }
}

impl SnakeState {
    pub fn new(width: i32, height: i32) -> Self {
        SnakeState {
            snake: Snake::new(Coord2D(
                           width.checked_div(2).unwrap(),
                           height.checked_div(2).unwrap())),
            board: Board::new(width, height),
        }
    }

    pub fn tick(&mut self) -> GameStatus {
        let new_head = self.snake.move_forward();
        if self.snake.collides_with_self()
            || self.board.collides_with(&new_head) {
            return GameStatus::GameOver
        } else if new_head == self.board.food_coord() {
            self.snake.grow();
            self.board.new_food();
        }
        GameStatus::Running
    }

    pub fn draw(&self) {
        let snake_set = self.snake.configuration();
        let boundary_set = self.board.configuration();
        let food = self.board.food_coord();

        let x_min = boundary_set.iter().min_by(|c1, c2| c1.0.cmp(&c2.0)).unwrap().0;
        let x_max = boundary_set.iter().max_by(|c1, c2| c1.0.cmp(&c2.0)).unwrap().0;
        let y_min = boundary_set.iter().min_by(|c1, c2| c1.1.cmp(&c2.1)).unwrap().1;
        let y_max = boundary_set.iter().max_by(|c1, c2| c1.1.cmp(&c2.1)).unwrap().1;

        for r in y_min..=y_max {
            for c in x_min..=x_max {
                let coord = Coord2D(c, r);
                if boundary_set.contains(&coord) {
                    print!("#");
                } else if coord == food {
                    print!("x");
                } else if snake_set.contains(&coord) {
                    print!("o");
                } else {
                    print!(" ");
                }
            }
            println!("");
        }
    }
}
