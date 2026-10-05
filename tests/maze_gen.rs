use maze_gen::rect_maze::{Direction, RectMaze};

#[test]
fn test_maze() {
    let mut maze = RectMaze::new(10, 10);
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


use maze_gen::circ_maze::{self, CircMaze};
use std::collections::HashSet;

/// Enumerate the neighbours of cell `(r, s)` (same geometry as the maze uses).
fn neighbors(maze: &CircMaze, r: usize, s: usize) -> Vec<(usize, usize)> {
    let sp = maze.spokes[r];
    let mut out = vec![(r, (s + 1) % sp), (r, (s + sp - 1) % sp)];
    if r > 0 {
        for k in maze.children(r, s) {
            out.push((r - 1, k));
        }
    }
    if r + 1 < maze.rings {
        out.push((r + 1, maze.parent[r][s]));
    }
    out
}

/// Is the wall between `a` and `b` open (i.e. removed)?
fn open_between(maze: &CircMaze, a: (usize, usize), b: (usize, usize)) -> bool {
    let (r, s) = a;
    let sp = maze.spokes[r];
    if b == (r, (s + 1) % sp) {
        return !maze.angular[r][s];
    }
    if b == (r, (s + sp - 1) % sp) {
        return !maze.angular[r][(s + sp - 1) % sp];
    }
    // `b` is in the outer ring relative to `a`.
    if b.0 + 1 == r {
        return !maze.radial[b.0][b.1];
    }
    // `b` is in the inner ring relative to `a`.
    if b.0 == r + 1 {
        return !maze.radial[r][s];
    }
    false
}

/// All cells reachable from `start` through open walls.
fn reachable(maze: &CircMaze, start: (usize, usize)) -> HashSet<(usize, usize)> {
    let mut seen = HashSet::new();
    let mut stack = vec![start];
    seen.insert(start);
    while let Some(cur) = stack.pop() {
        for n in neighbors(maze, cur.0, cur.1) {
            if seen.contains(&n) {
                continue;
            }
            if open_between(maze, cur, n) {
                seen.insert(n);
                stack.push(n);
            }
        }
    }
    seen
}

fn total_cells(maze: &CircMaze) -> usize {
    maze.spokes.iter().sum()
}

#[test]
fn test_circ_spokes_grow_linearly() {
    // inner_radius=0.5 puts ring mid radii at 1, 2, 3, ... so the counts are
    // exact multiples of the base.
    let maze = CircMaze::new(5, 8, 0.5);
    assert_eq!(maze.spokes, vec![40, 32, 24, 16, 8]);
}

#[test]
fn test_circ_layout_is_consistent() {
    for (rings, base, inner) in [(6, 8, 0.5), (12, 6, 1.0), (20, 5, 2.3), (8, 4, 0.0)] {
        let maze = CircMaze::new(rings, base, inner);
        assert_eq!(maze.spokes[rings - 1], base, "innermost keeps base");
        for r in 0..rings {
            let b = &maze.bounds[r];
            assert_eq!(b[0], 0.0);
            assert!(b.windows(2).all(|w| w[0] < w[1]), "bounds strictly increase");
        }
        for r in 0..rings - 1 {
            let (outer, inner_b) = (&maze.bounds[r], &maze.bounds[r + 1]);
            assert!(maze.spokes[r] >= maze.spokes[r + 1]);
            assert!(maze.spokes[r] <= 2 * maze.spokes[r + 1]);
            for (k, &p) in maze.parent[r].iter().enumerate() {
                // Each outer cell lies within its parent's angular extent.
                let p_end = inner_b.get(p + 1).copied().unwrap_or(1.0);
                let k_end = outer.get(k + 1).copied().unwrap_or(1.0);
                assert!(inner_b[p] <= outer[k] && k_end <= p_end);
            }
            // Every inner boundary is also an outer boundary (walls line up).
            for x in inner_b {
                assert!(outer.contains(x));
            }
        }
    }
}

#[test]
fn test_circ_maze_is_perfect_small() {
    let maze = circ_maze::generate(5, 8, 0.5, 0.5, 0.0);
    let reach = reachable(&maze, (0, 0));
    assert_eq!(reach.len(), total_cells(&maze));
}

#[test]
fn test_circ_maze_is_perfect_with_subdivision() {
    for (rings, base, inner) in [(4, 8, 0.5), (6, 6, 1.0), (8, 8, 0.0), (10, 4, 3.0), (12, 8, 0.7)] {
        let maze = circ_maze::generate(rings, base, inner, 0.3, 0.1);
        assert_eq!(maze.spokes.len(), rings);
        assert_eq!(maze.spokes[rings - 1], base, "innermost keeps base");
        for r in 1..rings {
            assert!(maze.spokes[r - 1] >= maze.spokes[r]);
        }
        let reach = reachable(&maze, (0, 0));
        assert_eq!(
            reach.len(),
            total_cells(&maze),
            "every cell reachable (rings={}, base={}, inner={})",
            rings,
            base,
            inner
        );
    }
}

#[test]
fn test_circ_solve_connects_start_and_stop() {
    let mut maze = circ_maze::generate(7, 8, 0.5, 0.5, 0.0);
    maze.open_start_and_end();
    let stop = (maze.rings - 1, maze.spokes[maze.rings - 1] / 2);
    circ_maze::solve(&mut maze, (0, 0), stop);
    assert!(maze.in_solution[0][0], "start should be on the path");
    assert!(maze.in_solution[stop.0][stop.1], "stop should be on the path");
    // Every solution-marked cell must be reachable through open walls.
    let reach = reachable(&maze, (0, 0));
    for r in 0..maze.rings {
        for s in 0..maze.spokes[r] {
            if maze.in_solution[r][s] {
                assert!(reach.contains(&(r, s)));
            }
        }
    }
}
