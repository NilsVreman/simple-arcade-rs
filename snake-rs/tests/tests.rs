use snake_rs::Snake;
use arcade_util::{Coord2D};

#[test]
fn test_move_forward() {
    let mut snake = Snake::new(Coord2D(0, 0));
    snake.move_forward();
    assert_eq!(snake.segments, vec![Coord2D(0, 1)]);
}
