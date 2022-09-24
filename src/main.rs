use maze_gen::dungeon::{
    generate as dungeon_generate, solve as dungeon_solve, Dungeon, RoomSettings,
};
use maze_gen::maze::{generate, solve, Maze};
pub fn main() {
    const WIDTH: usize = 32;
    const HEIGHT: usize = 32;
    let mut maze = Maze::new(WIDTH, HEIGHT);
    generate(&mut maze);
    solve(&mut maze, (0, 0), (WIDTH - 1, HEIGHT - 1));
    maze.print(Some("out.txt"), false).unwrap();

    let room_settings = RoomSettings {
        num_rooms: 4 * 32,
        max_width: 4,
        min_width: 2,
        max_height: 4,
        min_height: 2,
    };
    let mut dungeon = Dungeon::new(WIDTH, HEIGHT, Some(room_settings));
    dungeon_generate(&mut dungeon);
    //dungeon_solve(&mut dungeon, (0, 0), (WIDTH - 1, HEIGHT - 1));
    dungeon.print(Some("dungeon_out.txt"), false).unwrap();
}
