use std::collections::HashSet;
enum Direction {
    Right,
    Left,
    Up,
    Down
}

type Motion = (Direction, i32);
type Position = (i32, i32);
type World = (Position, Position, HashSet<Position>);

fn parse_motions(input: &String) -> Vec<Motion>{
    use Direction::*;
    input.trim().lines().map(|l| {
        let mut parts = l.split(" ");
        let dir = match parts.next().unwrap() {
            "R" => Right,
            "L" => Left,
            "U" => Up,
            "D" => Down,
            n => panic!("Unrecognized direction {n}")
        };
        let distance: i32 = parts.next().unwrap().parse().unwrap();
        (dir, distance)
    }).collect()
}

fn coord_delta(d: &Direction) -> Position {
    use Direction::*;
    match d {
        Right => (1, 0),
        Left => (-1, 0),
        Up => (0, -1),
        Down => (0, 1)
    }
}

fn tail_chase(head: Position, tail:Position) -> Position {
    let (head_x, head_y) = head;
    let (tail_x, tail_y) = tail;

    let diff_x = head_x - tail_x;
    let diff_y = head_y - tail_y;

    let mut out_x = tail_x;
    let mut out_y = tail_y;

    if diff_x.abs() <= 1 && diff_y.abs() <= 1 {
        return tail
    }

    if diff_x >= 2 {
        out_x = tail_x + 1;
        if diff_y != 0 {
            out_y = tail_y + diff_y.signum();
        }
    }
    if diff_x <= -2 {
        out_x = tail_x - 1;
        if diff_y != 0 {
            out_y = tail_y + diff_y.signum();
        }
    }
    if diff_y >= 2 {
        out_y = tail_y + 1;
        if diff_x != 0 {
            out_x = tail_x + diff_x.signum();
        }
    }
    if diff_y <= -2 {
        out_y = tail_y - 1;
        if diff_x != 0 {
            out_x = tail_x + diff_x.signum();
        }
    }

    (out_x, out_y)
}

fn update_world(world: World, motion: Motion) -> World {
    let (head, tail, mut visited) = world;
    let (dir, steps) = motion;
    let (mut head_x, mut head_y) = head;
    let (mut tail_x, mut tail_y) = tail;

    let (delta_x, delta_y) = coord_delta(&dir);

    for _ in 0..steps {
        head_x += delta_x;
        head_y += delta_y;

        (tail_x, tail_y) = tail_chase((head_x, head_y), (tail_x, tail_y));
        visited.insert((tail_x, tail_y));
    }

    ((head_x, head_y), (tail_x, tail_y), visited)
}

pub fn tail_visited_positions(input: &String) -> u32 {
    let motions = parse_motions(input);
    let mut world: World = ((0, 0), (0, 0), HashSet::new());

    for motion in motions {
        world = update_world(world, motion);
    }

    let (_, _, visited) = world;
    visited.len() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let input = r"
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
        ".to_string();
        assert_eq!(tail_visited_positions(&input), 13);
    }
}
