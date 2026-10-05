use rand::RngExt;
use rand::distr::Distribution;
use rand::distr::weighted::WeightedIndex;
use std::collections::{HashMap, HashSet};
use std::f64::consts::PI;
use std::io::Result;
use svg::Document;
use svg::node::element::Path;
use svg::node::element::path::Data;

/// Convert an SVG string to a standalone PDF buffer using the svg2pdf 0.13 API.
fn svg_to_pdf(svg: &str) -> std::result::Result<Vec<u8>, String> {
    let mut options = svg2pdf::usvg::Options::default();
    options.fontdb_mut().load_system_fonts();
    let tree = svg2pdf::usvg::Tree::from_str(svg, &options).map_err(|e| format!("{}", e))?;
    let pdf = svg2pdf::to_pdf(
        &tree,
        svg2pdf::ConversionOptions::default(),
        svg2pdf::PageOptions::default(),
    )
    .map_err(|e| format!("{:?}", e))?;
    Ok(pdf)
}

/// Cardinal direction within the maze, used for biasing generation.
#[derive(Copy, Clone, Debug)]
pub enum Direction {
    /// Toward the outer boundary (larger radius, ring index decreases).
    Out,
    /// Toward the center (smaller radius, ring index increases).
    In,
    /// Counter-clockwise angular neighbour (spoke index increases).
    Left,
    /// Clockwise angular neighbour (spoke index decreases).
    Right,
}

/// Identifier for a single removable wall segment between two adjacent cells.
#[derive(Copy, Clone, Debug)]
enum WallId {
    /// Angular wall `angular[ring][idx]`: the radial wall at the boundary
    /// between cell `(ring, idx)` and `(ring, idx+1)` (wrapping). It spans the
    /// full radial extent of `ring`.
    Angular { ring: usize, idx: usize },
    /// Radial wall `radial[outer][idx]`: the arc wall on the inner edge of
    /// outer cell `(outer, idx)`, i.e. the boundary between ring `outer` and
    /// ring `outer + 1`. It is stored per outer-ring cell because every outer
    /// cell lies entirely within a single inner (parent) cell.
    Radial { outer: usize, idx: usize },
}

pub struct CircMaze {
    pub rings: usize,
    /// Number of spokes of the innermost (smallest) ring. Outer rings get
    /// proportionally more, growing linearly with radius.
    pub base_spokes: usize,
    /// Radius of the central hole; ring `rings - 1` spans
    /// `inner_radius..inner_radius + 1`.
    pub inner_radius: f64,
    /// `spokes[r]` = number of cells in ring `r`. Ring `0` is the outermost.
    pub spokes: Vec<usize>,
    /// `bounds[r][s]`: start angle of cell `(r, s)` as a fraction of a full
    /// turn. The cell ends at `bounds[r][s + 1]` (or `1.0` for the last cell).
    /// Cells within a ring may differ in width.
    pub bounds: Vec<Vec<f64>>,
    /// `parent[r][k]` for `r in 0..rings-1`: index of the cell in ring `r + 1`
    /// that outer cell `(r, k)` sits on. Non-decreasing in `k`.
    pub parent: Vec<Vec<usize>>,
    /// `angular[r][s]`: wall between `(r, s)` and `(r, (s+1) % spokes[r])`.
    /// `true` means the wall is present.
    pub angular: Vec<Vec<bool>>,
    /// `radial[r][k]` for `r in 0..rings-1`: wall between outer cell `(r, k)`
    /// and inner cell `(r+1, parent[r][k])`. `true` = present.
    pub radial: Vec<Vec<bool>>,
    /// Outermost boundary arc per cell of ring `0`. `true` = wall present.
    pub outer_wall: Vec<bool>,
    /// Innermost boundary arc per cell of the last ring. `true` = wall present.
    pub inner_wall: Vec<bool>,
    /// `in_solution[r][s]`: whether cell `(r, s)` is on the solution path.
    pub in_solution: Vec<Vec<bool>>,
}

/// End angle (fraction of a turn) of cell `s` in a ring with start angles `bounds`.
fn cell_end(bounds: &[f64], s: usize) -> f64 {
    bounds.get(s + 1).copied().unwrap_or(1.0)
}

/// Decide which cells of a ring to split in half when growing outward.
///
/// `widths[s]` is the angular width (fraction of a turn) of cell `s`, in
/// angular order around the ring. Exactly `count` cells must be marked
/// (`count <= widths.len()`). Returns `split[s] == true` for each chosen cell.
fn choose_splits(widths: &[f64], count: usize) -> Vec<bool> {
    let mut split = vec![false; widths.len()];
    let mut remaining = count;
    let mut sorted = widths.to_vec();
    sorted.sort_by(|a, b| b.total_cmp(a));
    sorted.dedup();
    // Widest first; within a group of equal widths, pick evenly spaced cells.
    // Widths are exact dyadic fractions, so `==` groups ties reliably.
    for w in sorted {
        if remaining == 0 {
            break;
        }
        let tied: Vec<usize> = (0..widths.len()).filter(|&s| widths[s] == w).collect();
        let take = remaining.min(tied.len());
        for i in 0..take {
            split[tied[i * tied.len() / take]] = true;
        }
        remaining -= take;
    }
    split
}

/// Lay out the cells of every ring. The innermost ring has `base_spokes`
/// equal cells; each ring further out splits some of its inner neighbour's
/// cells in half so that the cell count grows linearly with the ring's mid
/// radius (keeping the arc width per cell roughly constant).
///
/// Returns `(bounds, parent)` indexed outermost-first, as stored in `CircMaze`.
fn compute_layout(
    rings: usize,
    base_spokes: usize,
    inner_radius: f64,
) -> (Vec<Vec<f64>>, Vec<Vec<usize>>) {
    if rings == 0 || base_spokes == 0 {
        return (
            vec![Vec::new(); rings],
            vec![Vec::new(); rings.saturating_sub(1)],
        );
    }
    let rho0 = inner_radius + 0.5;
    // Built innermost-first and reversed at the end.
    let mut bounds: Vec<Vec<f64>> = vec![
        (0..base_spokes)
            .map(|s| s as f64 / base_spokes as f64)
            .collect(),
    ];
    let mut parents: Vec<Vec<usize>> = Vec::new();
    for i in 1..rings {
        let inner = bounds.last().unwrap();
        let n_in = inner.len();
        let rho = rho0 + i as f64;
        let target = (base_spokes as f64 * rho / rho0).round() as usize;
        // Each cell can be split at most once per ring.
        let extra = target.saturating_sub(n_in).min(n_in);
        let widths: Vec<f64> = (0..n_in).map(|s| cell_end(inner, s) - inner[s]).collect();
        let split = choose_splits(&widths, extra);

        let mut outer = Vec::with_capacity(n_in + extra);
        let mut parent = Vec::with_capacity(n_in + extra);
        for s in 0..n_in {
            outer.push(inner[s]);
            parent.push(s);
            if split[s] {
                outer.push(inner[s] + widths[s] / 2.0);
                parent.push(s);
            }
        }
        bounds.push(outer);
        parents.push(parent);
    }
    bounds.reverse();
    parents.reverse();
    (bounds, parents)
}

impl CircMaze {
    pub fn new(rings: usize, base_spokes: usize, inner_radius: f64) -> CircMaze {
        let (bounds, parent) = compute_layout(rings, base_spokes, inner_radius);
        let spokes: Vec<usize> = bounds.iter().map(|b| b.len()).collect();
        let angular = spokes.iter().map(|&sp| vec![true; sp]).collect();
        let radial = (0..rings.saturating_sub(1))
            .map(|r| vec![true; spokes[r]])
            .collect();
        let outer_wall = vec![true; spokes.first().copied().unwrap_or(0)];
        let inner_wall = vec![true; spokes.last().copied().unwrap_or(0)];
        let in_solution = spokes.iter().map(|&sp| vec![false; sp]).collect();
        CircMaze {
            rings,
            base_spokes,
            inner_radius,
            spokes,
            bounds,
            parent,
            angular,
            radial,
            outer_wall,
            inner_wall,
            in_solution,
        }
    }

    /// Open the entrance (top of the outermost ring) and the exit (middle of
    /// the innermost ring).
    pub fn open_start_and_end(&mut self) {
        if self.rings == 0 {
            return;
        }
        // Entrance at the outer boundary of cell (0, 0).
        self.outer_wall[0] = false;
        // Exit at the inner boundary of the middle cell of the innermost ring.
        let mid = self.spokes[self.rings - 1] / 2;
        self.inner_wall[mid] = false;
    }

    /// Restore all walls and clear the solution marking.
    pub fn reset(&mut self) {
        for ring in self.angular.iter_mut() {
            for w in ring.iter_mut() {
                *w = true;
            }
        }
        for ring in self.radial.iter_mut() {
            for w in ring.iter_mut() {
                *w = true;
            }
        }
        for w in self.outer_wall.iter_mut() {
            *w = true;
        }
        for w in self.inner_wall.iter_mut() {
            *w = true;
        }
        for ring in self.in_solution.iter_mut() {
            for v in ring.iter_mut() {
                *v = false;
            }
        }
    }

    pub fn add_cell_to_solution(&mut self, ring: usize, spoke: usize) {
        self.in_solution[ring][spoke] = true;
    }

    /// Enumerate every adjacent cell of `(r, s)` together with the wall segment
    /// separating them and the direction of the neighbour. Across a subdivision
    /// boundary an inner-ring cell has two outward neighbours (one per outer
    /// half-arc), each with its own wall.
    fn neighbors(&self, r: usize, s: usize) -> Vec<((usize, usize), Direction, WallId)> {
        let sp = self.spokes[r];
        let mut out = Vec::with_capacity(6);

        // Angular neighbours (always exist; the ring wraps around).
        let left = (s + 1) % sp;
        out.push((
            (r, left),
            Direction::Left,
            WallId::Angular { ring: r, idx: s },
        ));
        let right = (s + sp - 1) % sp;
        out.push((
            (r, right),
            Direction::Right,
            WallId::Angular {
                ring: r,
                idx: right,
            },
        ));

        // Outward (toward ring `r - 1`, larger radius): the outer cells whose
        // parent is this cell (one, or two if this cell was split).
        if r > 0 {
            for k in self.children(r, s) {
                out.push((
                    (r - 1, k),
                    Direction::Out,
                    WallId::Radial {
                        outer: r - 1,
                        idx: k,
                    },
                ));
            }
        }

        // Inward (toward ring `r + 1`, smaller radius): the single parent cell.
        if r + 1 < self.rings {
            out.push((
                (r + 1, self.parent[r][s]),
                Direction::In,
                WallId::Radial { outer: r, idx: s },
            ));
        }

        out
    }

    /// Indices of the cells in ring `r - 1` that sit on cell `(r, s)`.
    /// Requires `r > 0`. Contiguous because `parent[r - 1]` is sorted.
    pub fn children(&self, r: usize, s: usize) -> std::ops::Range<usize> {
        let p = &self.parent[r - 1];
        p.partition_point(|&x| x < s)..p.partition_point(|&x| x <= s)
    }

    /// Angular extent `(start, end)` of cell `(r, s)` in radians, measured
    /// from the layout origin (before the drawing rotation is applied).
    fn cell_angles(&self, r: usize, s: usize) -> (f64, f64) {
        let b = &self.bounds[r];
        (2.0 * PI * b[s], 2.0 * PI * cell_end(b, s))
    }

    fn wall_present(&self, wall: WallId) -> bool {
        match wall {
            WallId::Angular { ring, idx } => self.angular[ring][idx],
            WallId::Radial { outer, idx } => self.radial[outer][idx],
        }
    }

    fn open_wall(&mut self, wall: WallId) {
        match wall {
            WallId::Angular { ring, idx } => self.angular[ring][idx] = false,
            WallId::Radial { outer, idx } => self.radial[outer][idx] = false,
        }
    }

    /// Render the maze to an SVG and a PDF file. When the maze has been
    /// solved (i.e. some cells are marked as part of the solution), an
    /// additional `sol_*.svg` / `sol_*.pdf` overlay showing the solution path
    /// in red is also written.
    pub fn draw(&self, path: Option<&str>, line_thickness: f64, transparency: f64) -> Result<()> {
        if self.rings == 0 || self.base_spokes == 0 {
            return Ok(());
        }

        let margin = 0.5;
        let outer_radius = self.rings as f64 + self.inner_radius;
        let translate = outer_radius + margin;
        // Choose the global angular origin so that outermost cell 0 is centred
        // at the top. All rings share this origin, and every outer boundary is
        // either an inner boundary or the midpoint of a split inner cell.
        let phi0 = -PI / 2.0 - self.cell_angles(0, 0).1 / 2.0;

        let to_pt = |rho: f64, phi: f64| -> (f64, f64) {
            (rho * phi.cos() + translate, rho * phi.sin() + translate)
        };

        let make_path = |data: Data| {
            Path::new()
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", line_thickness)
                .set("d", data)
        };

        let mut wall_paths = Vec::new();
        let mut solution_marks = Vec::new();

        for r in 0..self.rings {
            let sp = self.spokes[r];
            let rho_outer = outer_radius - r as f64;
            let rho_inner = outer_radius - r as f64 - 1.0;
            for s in 0..sp {
                let (a_lo, a_hi) = self.cell_angles(r, s);
                let (phi_lo, phi_hi) = (a_lo + phi0, a_hi + phi0);

                // Left radial wall: the angular boundary between this cell and
                // the next (counter-clockwise). Drawn once per cell, so every
                // angular wall is drawn exactly once.
                if self.angular[r][s] {
                    let p1 = to_pt(rho_inner, phi_hi);
                    let p2 = to_pt(rho_outer, phi_hi);
                    let data = Data::new().move_to(p1).line_to(p2);
                    wall_paths.push(make_path(data));
                }

                // Inner arc of this cell.
                if r == self.rings - 1 {
                    // Innermost ring: the inner boundary (the hole edge).
                    if rho_inner > 0.0 && self.inner_wall[s] {
                        let p1 = to_pt(rho_inner, phi_lo);
                        let p2 = to_pt(rho_inner, phi_hi);
                        let data = Data::new()
                            .move_to(p1)
                            .elliptical_arc_to((rho_inner, rho_inner, 0.0, 0, 1, p2.0, p2.1));
                        wall_paths.push(make_path(data));
                    }
                } else {
                    // Boundary between this ring and the next inner one. Stored
                    // per outer cell, so drawing it here (and skipping the outer
                    // arc of the inner ring) renders every radial segment once.
                    if self.radial[r][s] {
                        let p1 = to_pt(rho_inner, phi_lo);
                        let p2 = to_pt(rho_inner, phi_hi);
                        let data = Data::new()
                            .move_to(p1)
                            .elliptical_arc_to((rho_inner, rho_inner, 0.0, 0, 1, p2.0, p2.1));
                        wall_paths.push(make_path(data));
                    }
                }

                // Outer arc: only the outermost ring has a real outer boundary;
                // inner rings' outer edges are drawn as the inner arcs above.
                if r == 0 && self.outer_wall[s] {
                    let p1 = to_pt(rho_outer, phi_lo);
                    let p2 = to_pt(rho_outer, phi_hi);
                    let data = Data::new()
                        .move_to(p1)
                        .elliptical_arc_to((rho_outer, rho_outer, 0.0, 0, 1, p2.0, p2.1));
                    wall_paths.push(make_path(data));
                }

                // Solution mark: fill the whole annular sector of the cell.
                if self.in_solution[r][s] {
                    let po1 = to_pt(rho_outer, phi_lo);
                    let po2 = to_pt(rho_outer, phi_hi);
                    let pi1 = to_pt(rho_inner, phi_hi);
                    let pi2 = to_pt(rho_inner, phi_lo);
                    let mut data = Data::new()
                        .move_to(po1)
                        .elliptical_arc_to((rho_outer, rho_outer, 0.0, 0, 1, po2.0, po2.1))
                        .line_to(pi1);
                    if rho_inner > 0.0 {
                        data =
                            data.elliptical_arc_to((rho_inner, rho_inner, 0.0, 0, 0, pi2.0, pi2.1));
                    } else {
                        // Degenerate inner edge (inner_radius == 0): collapse to
                        // the centre point rather than emitting a zero-radius arc.
                        data = data.line_to(pi2);
                    }
                    data = data.close();
                    solution_marks.push(
                        Path::new()
                            .set("fill", "red")
                            .set("stroke", "none")
                            .set("opacity", transparency)
                            .set("d", data),
                    );
                }
            }
        }

        let base = path.unwrap_or("maze");

        // Main maze image: walls only.
        let mut document =
            Document::new().set("viewBox", (0.0, 0.0, 2.0 * translate, 2.0 * translate));
        for p in &wall_paths {
            document = document.add(p.clone());
        }
        svg::save(format!("{base}.svg"), &document).unwrap();
        let svg = std::fs::read_to_string(format!("{base}.svg")).unwrap();
        let ok_pdf = match svg_to_pdf(&svg) {
            Ok(pdf) => {
                std::fs::write(format!("{base}.pdf"), pdf).unwrap();
                true
            }
            Err(e) => {
                println!("Error: {e}, could not produce PDF");
                false
            }
        };

        // Solved overlay: solution marks drawn on top of the walls. Only
        // emitted when there is actually a solution to show.
        if !solution_marks.is_empty() {
            let mut document =
                Document::new().set("viewBox", (0.0, 0.0, 2.0 * translate, 2.0 * translate));
            for m in &solution_marks {
                document = document.add(m.clone());
            }
            for p in &wall_paths {
                document = document.add(p.clone());
            }
            svg::save(format!("sol_{base}.svg"), &document).unwrap();
            if ok_pdf {
                let svg = std::fs::read_to_string(format!("sol_{base}.svg")).unwrap();
                match svg_to_pdf(&svg) {
                    Ok(pdf) => std::fs::write(format!("sol_{base}.pdf"), pdf).unwrap(),
                    Err(e) => println!("Error: {e}, could not produce solution PDF"),
                }
            }
        }
        Ok(())
    }
}

/// Generate a circular maze using the recursive backtracker algorithm.
///
/// `base_spokes` is the number of cells in the innermost ring; outer rings
/// split some of their inner neighbour's cells so the cell count grows
/// linearly with radius and the arc width of a cell stays roughly constant.
pub fn generate(
    rings: usize,
    base_spokes: usize,
    inner_radius: f64,
    bias: f64,
    length_bias: f64,
) -> CircMaze {
    let mut maze = CircMaze::new(rings, base_spokes, inner_radius);
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut stack: Vec<(usize, usize)> = Vec::new();

    let angle_weight = 1.0 - bias;
    let radial_weight = bias;
    let mut rng = rand::rng();

    let start = (0, 0);
    visited.insert(start);
    stack.push(start);

    while let Some((r, s)) = stack.pop() {
        // Collect unvisited neighbours with their separating wall, direction
        // (for biasing) and weight.
        let mut candidates: Vec<(WallId, (usize, usize), f64)> = Vec::new();
        for (n, dir, wall) in maze.neighbors(r, s) {
            if visited.contains(&n) {
                continue;
            }
            let weight = match dir {
                Direction::In => radial_weight,
                Direction::Out => radial_weight + length_bias,
                Direction::Left | Direction::Right => angle_weight + length_bias,
            };
            candidates.push((wall, n, weight));
        }

        if !candidates.is_empty() {
            // Keep this cell on the stack so we can backtrack to it later.
            stack.push((r, s));

            let weights: Vec<f64> = candidates.iter().map(|c| c.2).collect();
            let pick = match WeightedIndex::new(&weights) {
                Ok(dist) => dist.sample(&mut rng),
                // Fall back to a uniform choice if every weight is zero.
                Err(_) => rng.random_range(0..candidates.len()),
            };
            let (wall, n, _) = candidates[pick];
            maze.open_wall(wall);
            visited.insert(n);
            stack.push(n);
        }
    }

    maze
}

/// Solve the maze by finding the unique path from `start` to `stop` (a perfect
/// maze has exactly one such path) and mark every cell along it.
pub fn solve(maze: &mut CircMaze, start: (usize, usize), stop: (usize, usize)) {
    // Depth-first search from `start`, recording the parent of each reached
    // cell. We only traverse through openings (walls that have been removed).
    let mut parent: HashMap<(usize, usize), Option<(usize, usize)>> = HashMap::new();
    let mut stack: Vec<(usize, usize)> = vec![start];
    parent.insert(start, None);

    while let Some(cur) = stack.pop() {
        if cur == stop {
            break;
        }
        for (n, _dir, wall) in maze.neighbors(cur.0, cur.1) {
            // A present wall blocks travel; an open one (false) connects.
            if maze.wall_present(wall) {
                continue;
            }
            if parent.contains_key(&n) {
                continue;
            }
            parent.insert(n, Some(cur));
            stack.push(n);
        }
    }

    // Walk back from `stop` to `start` using the parent pointers, marking each
    // cell as part of the solution.
    let mut cur: Option<(usize, usize)> = Some(stop);
    while let Some(c) = cur {
        maze.add_cell_to_solution(c.0, c.1);
        cur = parent.get(&c).copied().flatten();
    }
}
