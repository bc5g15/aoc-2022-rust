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
type ManyTailWorld = (Position, Vec<Position>, HashSet<Position>);

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

fn update_many_tail_world(world: ManyTailWorld, motion: Motion) -> ManyTailWorld {
    let (head, mut tails, mut visited) = world;
    let (dir, steps) = motion;
    let (mut head_x, mut head_y) = head;

    let (delta_x, delta_y) = coord_delta(&dir);

    for _ in 0..steps {
        head_x += delta_x;
        head_y += delta_y;

        let mut last_head_x = head_x;
        let mut last_head_y = head_y;
        tails = tails.iter().map(|(tx,ty)| {
            let (nx, ny) = tail_chase((last_head_x, last_head_y), (*tx, *ty));
            last_head_x = nx;
            last_head_y = ny;
            (nx, ny)
        }).collect();
        visited.insert((last_head_x, last_head_y));
    }

    ((head_x, head_y), tails, visited)
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

pub fn many_tail_visited_positions(input: &String) -> u32 {
    let motions = parse_motions(input);
    let head = (0, 0);
    let tails: Vec<Position> = (0..9).map(|_| (0, 0)).collect();

    let mut world: ManyTailWorld = (head, tails, HashSet::new());

    for motion in motions {
        world = update_many_tail_world(world, motion);
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

    #[test]

    fn part_two_short() {
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
        assert_eq!(many_tail_visited_positions(&input), 1);
    }

    #[test]
    fn part_two_long() {
        let input = r"
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
    ".to_string();

    assert_eq!(many_tail_visited_positions(&input), 36);
    }
}
