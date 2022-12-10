enum Instruction {
    AddX (i32),
    Noop
}

fn parse_instruction(input: &str) -> Instruction {
    let parts: Vec<&str> = input.split(" ").collect();
    match parts.get(0).unwrap() {
        &"addx" => {
            let value: i32 = parts.get(1).unwrap().parse().unwrap();
            return Instruction::AddX(value);
        },
        &"noop" => {
            return Instruction::Noop;
        },
        _ => panic!("Unrecognised line {input}")
    }
}

pub fn sample_at_points(input: &String) -> i32 {
    let code: Vec<Instruction> = input.trim().lines().map(|l| parse_instruction(l)).collect();
    let mut x = 1;
    let mut signal_sum = 0;
    let mut code_loop = code.iter().cycle();
    let mut clock = 0;

    let mut check_add = |value: i32, clock: u32| {
        if clock % 40 == 20 {
            signal_sum += (clock as i32) * value;
        }
    };

    while clock < 221 {
        let inst = code_loop.next().unwrap();
        match inst {
            Instruction::Noop => {
                clock += 1;
                check_add(x, clock);
            },
            Instruction::AddX(v) => {
                for _ in 0..2 {
                    clock += 1;
                    check_add(x, clock);
                }
                x += v;
            }
        }
    }
    signal_sum
}

fn process_time(instruction: &Instruction) -> u32 {
    use Instruction::*;
    match instruction {
        Noop => 1,
        AddX (_) => 2
    }
}

struct Process {
    timer: u32,
    is_complete: bool,
    result: i32
}

impl Process {
    fn new(instruction: &Instruction) -> Self {
        Self {
            timer: process_time(&instruction),
            result : match instruction {
                Instruction::Noop => 0,
                Instruction::AddX(v) => *v
            },
            is_complete: false
        }
    }
    fn run(&mut self) -> Option<i32>{
        self.timer -= 1;
        if self.timer > 0 {
            return None;
        }
        self.is_complete = true;
        Some(self.result)
    }
}

pub fn get_whole_image(input: &String) {
    let code: Vec<Instruction> = input.trim().lines().map(|l| parse_instruction(l)).collect();
    let mut code_loop = code.iter().cycle();
    let mut x: i32 = 1;

    let mut line: Vec<char> = Vec::new();
    let mut image: Vec<String> = Vec::new();

    let mut process: Process = Process::new(code_loop.next().unwrap());

    // lines
    for _ in 0..6 {
        // cells
        for i in 0..40 {
            // Drawing
            line.push(if (x-i).abs() <= 1 {'#'} else {' '});

            // Processing
            let r = process.run();
            if let Some(v) = r {
                x += v;
                process = Process::new(code_loop.next().unwrap());
            }
        }
        image.push(line.iter().collect());
        line = Vec::new();
    }

    let final_string = image.join("\n");
    println!("{final_string}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn long_example() {
        let input = r"
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop".to_string();

        let result = sample_at_points(&input);
        assert_eq!(result, 13140);
    }
}
