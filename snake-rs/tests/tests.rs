use snake_rs::Snake;
use arcade_util::{Coord2D, Rot2D};

#[test]
fn test_move_forward() {
    let mut snake = Snake::new(Coord2D(0, 0));
    snake.move_forward();
    assert_eq!(snake.segments, vec![Coord2D(0, 1)]);
}

#[test]
fn test_rotate() {
    let mut snake = Snake::new(Coord2D(0, 0));
    snake.rotate(&Rot2D::Clockwise);
    snake.move_forward();
    assert_eq!(snake.segments, vec![Coord2D(1, 0)]);
}

#[test]
fn test_self_collision() {
    let mut snake = Snake::new(Coord2D(0, 0));
    snake.grow();
    todo!();
    snake.move_forward();
    assert_eq!(snake.segments, vec![Coord2D(1, 0)]);
}
