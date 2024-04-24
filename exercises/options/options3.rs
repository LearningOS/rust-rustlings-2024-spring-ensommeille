// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.

// ref
use std::ops::Deref;

struct Point {
    x: i32,
    y: i32,
}

pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

impl Deref for Command {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        match self {
            Command::Uppercase => &0,
            Command::Trim => &0,
            Command::Append(size) => size,
        }
    }
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match y {
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => panic!("no match!"),
    }
    y; // Fix without deleting this line.

    let input = vec![
        ("  hello  ".to_string(), Command::Trim),
        ("hello".to_string(), Command::Uppercase),
        ("world".to_string(), Command::Append(2)),
    ];

    let mut output = Vec::new();

    for (string, command) in input.iter() {
        match command {
        //隐式引用捕获
            Command::Uppercase => output.push(string.to_uppercase()),
            Command::Trim => output.push(string.trim().to_string()),
            Command::Append(size) => {
                let mut appended = string.clone();
        //size为&usize类型
                for _ in 0..*size {
                    appended.push_str("bar");
                }
                output.push(appended);
            }
        }
    }
}
