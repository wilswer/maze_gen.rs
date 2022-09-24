use maze_gen::maze::Maze;
use maze_gen::utils::Direction;

#[test]
fn test_maze() {
    let mut maze = Maze::new(10, 10);
    assert_eq!(maze.width, 10);
    assert_eq!(maze.height, 10);
    maze.set(0, 0, &Direction::Up, false);
    assert_eq!(maze.get(0, 0).up, false);
    maze.set(9, 9, &Direction::Down, false);
    maze.reset();
    assert_eq!(maze.get(9, 9).down, true);
    maze.add_cell_to_solution(0, 0);
    assert_eq!(maze.get(0, 0).in_solution, true);
}
