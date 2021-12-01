use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

struct Window {
    first: Option<i32>,
    second: Option<i32>,
    third: Option<i32>,
}

impl Window {
    fn empty() -> Window {
        Window {
            first: None,
            second: None,
            third: None,
        }
    }

    fn push(&self, value: i32) -> Window {
        Window {
            first: self.second,
            second: self.third,
            third: Option::Some(value),
        }
    }

    fn total(&self) -> i32 {
        self.first.unwrap() + self.second.unwrap() + self.third.unwrap()
    }

    fn is_valid(&self) -> bool {
        self.first.is_some() && self.second.is_some() && self.third.is_some()
    }
}

fn main() {
    if let Ok(lines) = read_lines("./input/1.txt") {
        let mut prev_depth = 99999;
        let mut count = 0;
        let mut window = Window::empty();

        for line in lines {
            if let Ok(depth_str) = line {
                let depth = depth_str.parse::<i32>().unwrap();
                window = window.push(depth);

                if !window.is_valid() {
                    continue;
                }

                let total = window.total();

                if total > prev_depth {
                    count = count + 1;
                }
                prev_depth = total;
            }
        }
        println!("{}", count);
    }
}

// Task 1
// fn main() {
//     if let Ok(lines) = read_lines("./input/1.txt") {
//         let mut prev_depth = 99999;
//         let mut count = 0;

//         for line in lines {
//             if let Ok(depth_str) = line {
//                 let depth = depth_str.parse::<i32>().unwrap();
//                 if depth > prev_depth {
//                     count = count + 1;
//                 }
//                 prev_depth = depth;
//             }
//         }
//         println!("{}", count);
//     }
// }

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
