type Guide = Vec<(char, char)>;

fn read_guide(input: &String) -> Guide{
    input.trim().lines().map(|l| {
        let mut chars = l.chars();
        let lhs = chars.next().expect("Should be a first character");
        chars.next();
        let rhs = chars.next().expect("Should be a third character");
        (lhs, rhs)
    }).collect()
}

fn turn_score(game: (char, char)) -> u32 {
    // A: Rock B: Paper C: Scissors
    // X: Rock Y: Paper Z: Scissors
    // R: 1 P: 2 S: 3
    // L: 0 D: 3 W: 6
    let (_, you) = game;
    let mut score = 0;
    score += match you {
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        _ => panic!("Unknown hand {you}")
    };

    score += match game  {
        ('A', 'X') => 3,
        ('B', 'Y') => 3,
        ('C', 'Z') => 3,
        ('A', 'Z') => 0,
        ('B', 'X') => 0,
        ('C', 'Y') => 0,
        ('A', 'Y') => 6,
        ('B', 'Z') => 6,
        ('C', 'X') => 6,
        _ => panic!("Invalid game {game:?}")
    };

    score
}

fn game_score(guide: Guide) -> u32 {
    guide.iter().map(|g| turn_score(*g)).sum()
}

pub fn guide_score(input: &String) -> u32 {
    let g = read_guide(input);
    game_score(g)
}

fn turn_conversion(turn: (char, char)) -> (char, char) {
    // A: Rock B: Paper C: Scissors
    // in   - X: lose Y: draw Z: win
    // out  - X: Rock Y: Paper Z: Scissors
    let (them, _) = turn;
    let second = match turn {
        ('A', 'X') => 'Z',
        ('B', 'X') => 'X',
        ('C', 'X') => 'Y',
        ('A', 'Y') => 'X',
        ('B', 'Y') => 'Y',
        ('C', 'Y') => 'Z',
        ('A', 'Z') => 'Y',
        ('B', 'Z') => 'Z',
        ('C', 'Z') => 'X',
        _ => panic!("Unrecognized game {turn:?}")
    };
    (them, second)
}

pub fn true_guide_score(input: &String) -> u32 {
    let g = read_guide(input);
    let true_g: Guide = g.iter().map(|t| turn_conversion(*t)).collect();
    game_score(true_g)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let input = "A Y\nB X\nC Z".to_string();
        let score = guide_score(&input);
        assert_eq!(score, 15);
    }

    #[test]
    fn part_two() {
        let input = "A Y\nB X\nC Z".to_string();
        let score = true_guide_score(&input);
        assert_eq!(score, 12)
    }
}