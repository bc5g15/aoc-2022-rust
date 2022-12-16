use std::collections::HashSet;
use std::cmp::{min, max};

type Coord = (i32, i32);
type World = HashSet<Coord>; 

fn build_line(start: Coord, end: Coord) -> World {
    let (a, b) = start;
    let (x, y) = end;

    let mut output: World = HashSet::new();

    if a == x {
        for i in min(b, y)..=max(b,y) {
            output.insert((a, i));
        }
    } else if b == y {
        for i in min(a, x)..=max(a, x) {
            output.insert((i, b));
        }
    }
    output

}

fn read_rocks(input: &String) -> World {
    let mut world: World = HashSet::new();
    input.trim().lines().for_each(|line| {
        let coords: Vec<Coord> = line.split(" -> ").map(|p| {
            let mut parts = p.split(",");
            let l: i32 = parts.next().unwrap().parse().unwrap();
            let r: i32 = parts.next().unwrap().parse().unwrap();
            (l, r)
        }).collect();

        for i in 1..coords.len() {
            let left = coords[i-1];
            let right = coords[i];
            let line = build_line(left, right);
            world = world.union(&line).map(|c| *c).collect();
        }
    });
    world
}

fn find_lowest_y(world: &World) -> i32{
    world.iter().map(|(_x, y)| *y).max().unwrap()
}

fn compute_sand_position(rock: &World, sand: &World, low_point: i32) -> Option<Coord>{
    let mut x: i32 = 500;
    let mut y: i32 = 0;

    loop {
        y += 1;

        // Check sand collision
        if sand.contains(&(x, y)) || rock.contains(&(x, y)){
            // Try to the left
            if !sand.contains(&(x-1, y))  && !rock.contains(&(x-1, y)) {
                x-=1;
                continue;
            }
            // Try to the right
            if !sand.contains(&(x+1, y)) && !rock.contains(&(x+1, y)) {
                x+=1;
                continue;
            }
            // Otherwise we rest on top
            return Some((x, y-1));
        }

        // Are we below everything?
        if y > low_point {
            return None;
        }
    }
}

fn simulate_sand(rock: &World) -> u32 {
    let mut sand: World = HashSet::new();

    let mut count: u32 = 0;
    let low_point = find_lowest_y(&rock);

    loop {
        count += 1;

        match compute_sand_position(&rock, &sand, low_point) {
            Some(coord) => { 
                sand.insert(coord); 
            }
            None => return count-1
        }
    }
}

pub fn maximum_static_sand(input: &String) -> u32 {
    let rock = read_rocks(input);
    simulate_sand(&rock)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let input = r"
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
        ".to_string();

        assert_eq!(maximum_static_sand(&input), 24);
    }
}