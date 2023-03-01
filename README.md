# Maze Generator

This is a simple maze generator written in Rust. It uses the
[recursive backtracker algorithm](https://en.wikipedia.org/wiki/Maze_generation_algorithm#Recursive_backtracker)
to generate mazes.

## Example with solution marked by "x":

```text
┌   ┬---┬---┬---┬---┬---┬---┬---┬---┬---┬---┬---┬---┬---┬---┬---┐
│ x   x   x │                     x   x │     x   x │           │
├---┼---┼   ┼---┼---┼---┼---┼---┼   ┼   ┼---┼   ┼   ┼   ┼   ┼---┤
│       │ x   x   x   x   x   x   x │ x   x │ x │ x │   │       │
├   ┼---┼---┼---┼---┼---┼---┼---┼---┼---┼   ┼   ┼   ┼---┼---┼   ┤
│ x   x   x   x │ x   x │ x   x   x     │ x │ x │ x │ x   x   x │
├   ┼---┼---┼   ┼   ┼   ┼   ┼---┼   ┼---┼   ┼   ┼   ┼   ┼---┼   ┤
│ x │       │ x   x │ x │ x │   │ x │ x   x │ x │ x   x │   │ x │
├   ┼   ┼   ┼---┼---┼   ┼   ┼   ┼   ┼   ┼---┼   ┼---┼---┼   ┼   ┤
│ x │   │           │ x │ x   x │ x   x │ x   x │           │ x │
├   ┼   ┼   ┼---┼---┼   ┼---┼   ┼---┼---┼   ┼---┼   ┼---┼---┼   ┤
│ x │   │           │ x   x   x │     x   x │       │ x   x   x │
├   ┼---┼---┼---┼   ┼---┼---┼---┼   ┼   ┼---┼   ┼---┼   ┼---┼---┤
│ x   x   x   x   x   x │           │ x │   │       │ x   x   x │
├---┼---┼---┼---┼---┼   ┼---┼---┼   ┼   ┼   ┼   ┼   ┼---┼---┼   ┤
│           │ x   x   x │ x   x │   │ x │   │   │           │ x │
├   ┼   ┼---┼   ┼---┼---┼   ┼   ┼---┼   ┼   ┼   ┼---┼   ┼---┼   ┤
│   │         x   x   x   x │ x   x   x │           │         x │
└---┴---┴---┴---┴---┴---┴---┴---┴---┴---┴---┴---┴---┴---┴---┴   ┘
```
