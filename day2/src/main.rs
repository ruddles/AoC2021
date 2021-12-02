use std::{
    fs::File,
    io::{self, BufRead},
    panic,
    path::Path,
};
#[derive(Debug)]
enum Instruction {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl Instruction {
    fn parse(value: String) -> Instruction {
        let values = value.split(" ").collect::<Vec<&str>>();
        if values.len() != 2 {
            panic!("Invalid line '{}'", value);
        }

        let direction = values[0];
        let distance = values[1].parse::<i32>().unwrap();

        match direction {
            "forward" => Instruction::Forward(distance),
            "down" => Instruction::Down(distance),
            "up" => Instruction::Up(distance),
            _ => panic!("unknown direction {}", direction),
        }
    }
}

struct Route {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

impl Route {
    fn new() -> Route {
        Route {
            horizontal: 0,
            depth: 0,
            aim: 0,
        }
    }

    // Task 1
    // fn process(&mut self, instruction: Instruction) {
    //     match instruction {
    //         Instruction::Forward(d) => self.horizontal = self.horizontal + d,
    //         Instruction::Down(d) => self.depth = self.depth + d,
    //         Instruction::Up(d) => self.depth = self.depth - d,
    //     };
    // }

    fn process(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Forward(d) => {
                self.horizontal = self.horizontal + d;
                self.depth = self.depth + (self.aim * d)
            }
            Instruction::Down(d) => self.aim = self.aim + d,
            Instruction::Up(d) => self.aim = self.aim - d,
        };
    }

    fn result(&self) -> i32 {
        self.depth * self.horizontal
    }
}

fn main() {
    if let Ok(lines) = read_lines("./input/1.txt") {
        let mut route = Route::new();

        for line in lines {
            if let Ok(instruction_str) = line {
                let instruction = Instruction::parse(instruction_str);
                route.process(instruction);
            }
        }

        println!("{}", route.result())
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
