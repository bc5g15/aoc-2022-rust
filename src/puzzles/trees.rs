
type TreeGrid = Vec<Vec<u32>>;

fn read_tree_grid(input: &String) -> TreeGrid{
    input.trim().lines().map(|l| {
        l.chars().map(|c| c.to_digit(10).unwrap()).collect()
    }).collect()
}

fn can_see_edge(row: usize, column: usize, grid: &TreeGrid) -> bool{
    let value = grid[row][column];
    let height = grid.len();
    let width = grid[row].len();

    let left    = (0..column).rev().all(|temp_column| grid[row][temp_column] < value);
    let right   = (column+1..width).all(|temp_column| grid[row][temp_column] < value);
    let up      = (0..row).rev().all(|temp_row| grid[temp_row][column] < value);
    let down    = (row+1..height).rev().all(|temp_row| grid[temp_row][column] < value);

    left || right || up || down
}

fn trees_on_path(path: &Vec<(usize, usize)>, grid: &TreeGrid, start_value: u32) -> u32 {
    let mut seen_count: u32 = 0;
    for i in 0..path.len() {
        let (row, column) = path[i];
        let seen_value = grid[row][column];
        if seen_value < start_value {
            seen_count += 1;
        } else {
            seen_count += 1;
            return seen_count;
        }
    }
    seen_count
}

fn scenic_score(row: usize, column: usize, grid: &TreeGrid) -> u32 {
    let value = grid[row][column];
    let height = grid.len();
    let width = grid[row].len();

    let quick_trees = |path: &Vec<(usize, usize)>| trees_on_path(path, grid, value);

    let left    = quick_trees(&(0..column).rev().map(|temp_column| (row, temp_column)).collect());
    let right   = quick_trees(&(column+1..width).map(|temp_column| (row, temp_column)).collect());
    let up      = quick_trees(&(0..row).rev().map(|temp_row| (temp_row, column)).collect());
    let down    = quick_trees(&(row+1..height).map(|temp_row| (temp_row, column)).collect());

    left * right * up * down
}

fn count_edgeview_trees(grid: &TreeGrid) -> u32{
    grid.iter().enumerate().map(|(row, cells)| 
        cells.iter().enumerate().filter(|(column, _value)| can_see_edge(row, *column, grid)).count() as u32
    ).sum()
}

pub fn count_visible_trees(input: &String) -> u32 {
    let grid = read_tree_grid(input);
    count_edgeview_trees(&grid)
}

pub fn best_scenic_score(input: &String) -> u32{
    let grid = read_tree_grid(input);
    grid.iter().enumerate().map(|(row, cells)|
        cells.iter().enumerate().map(|(column, _value)| scenic_score(row, column, &grid)).max().unwrap())
        .max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let input = r"
30373
25512
65332
33549
35390".to_string();

        let grid = read_tree_grid(&input);
        let edge_count = count_edgeview_trees(&grid);
        assert_eq!(edge_count, 21);
    }

    #[test]
    fn part_two() {
        let input = r"
30373
25512
65332
33549
35390".to_string();

        assert_eq!(best_scenic_score(&input), 8);
    }
}
