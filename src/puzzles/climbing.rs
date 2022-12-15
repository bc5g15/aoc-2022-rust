use std::collections::{HashSet, VecDeque};
use std::cmp::min;

type HeightGrid = Vec<Vec<u8>>;
type Coord = (usize, usize);
//   World = (Map, Start, End);
type World = (HeightGrid, Coord, Coord);

fn parse_elevations(input: &String) -> World {
    let mut start_point: Coord = (0, 0);
    let mut end_point: Coord = (0, 0);
    let grid = input.trim().lines().enumerate().map(|(row, l)| l.bytes()
        .enumerate().map(|(column, b)|{
            if b == b'S' {
                start_point = (row, column);
                return 0;
            }
            if b == b'E' {
                end_point = (row, column);
                return b'z'-b'a';
            }
            b - b'a'
        }).collect()).collect();
    (grid, start_point, end_point)
}

fn taxi_distance((a, b): Coord, (x, y): Coord) -> usize {
    a.abs_diff(x) + b.abs_diff(y)
}

fn get_surrounds((row, column): Coord, width: usize, height: usize) -> Vec<Coord> {
    let mut output: Vec<Coord> = Vec::new();
    if let Some(v) = row.checked_sub(1) {
        output.push((v, column));
    }
    if let Some(v) = column.checked_sub(1) {
        output.push((row, v));
    }
    if row < height - 1{
        output.push((row+1, column));
    }
    if column < width - 1 {
        output.push((row, column+1));
    }
    output
}

// (Current, Visited, Current Cost, Estimated Cost)
type SearchNode = (Coord, u32, usize);
fn heuristic_search(w: World, starting_nodes: Vec<SearchNode>) -> u32{
    let (grid, start, end) = w;
    let height = grid.len();
    let width = grid[0].len();
    let mut nodes: VecDeque<SearchNode> = VecDeque::from(starting_nodes);
    let cost_guess = taxi_distance(start, end);
    nodes.push_back((start, 0, cost_guess));

    let mut visited: HashSet<Coord> = HashSet::new();

    let mut best_steps: u32 = u32::MAX;

    while nodes.len() > 0 {
        let (pos, steps, _) = nodes.pop_front().unwrap();
        if visited.contains(&pos) {
            continue;
        }
        if pos == end {
            best_steps = min(best_steps, steps);
        } else {
            visited.insert(pos);
        }
        let (row, column) = pos;
        let current_height = grid[row][column];

        let surrounds = get_surrounds(pos, width, height);
        
        surrounds.iter().filter(|cs| {
            let (r, c) = cs;
            let new_height = grid[*r][*c];
            new_height <= current_height + 1 &&
            !visited.contains(cs) 
        }).for_each(|cs| {
            nodes.push_back((
                *cs,
                steps+1,
                (steps as usize) + taxi_distance(*cs, end)
            ));
        });

        nodes.make_contiguous().sort_by_key(|(_, _, cost_guess)|  *cost_guess);

    }
    
    best_steps
}

pub fn shortest_path(input: &String) -> u32 {
    let world = parse_elevations(&input);
    heuristic_search(world, vec![])
}

pub fn shortest_from_low(input: &String) -> u32 {
    let world = parse_elevations(&input);
    let mut lowest_points: Vec<SearchNode> = Vec::new();
    world.0.iter().enumerate().for_each(|(row, line)| {
        line.iter().enumerate().for_each(|(column, v)|{
            if *v == 0 {
                lowest_points.push(((row, column), 0, taxi_distance((row, column), world.2)));
            }
        })
    });
    heuristic_search(world, lowest_points)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let input = r"
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi".to_string();

        assert_eq!(shortest_path(&input), 31);
        assert_eq!(shortest_from_low(&input), 29);
    }
}