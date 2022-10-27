use std::str::FromStr;
use std::fmt::Debug;

pub struct Config {
    pub board_width: usize,
    pub board_height: usize,
    pub speed: u64,
    pub num_food: u32,
}

impl Config {
    pub fn new(args: &[String]) -> Self {
        let len = args.len();
        let use_args = len == 5;

        if !use_args && len != 1 {
            println!("USAGE: {} [width] [height] [speed] [num_food]", args[0]);
            std::process::exit(-1);
        }

        let board_width = if use_args {
            args[1].parse().expect("Failed to parse width")
        } else {
            Self::input("Enter a board width:")
        };

        let board_height = if use_args {
            args[2].parse().expect("Failed to parse height")
        } else {
            Self::input("Enter a board height:")
        };

        let speed = if use_args {
            args[3].parse().expect("Failed to parse speed")
        } else {
            Self::input("Enter a speed:")
        };

        let num_food = if use_args {
            args[4].parse().expect("Failed to parse number of food")
        } else {
            Self::input("Enter number of food:")
        };

        Self {
            board_width,
            board_height,
            speed,
            num_food,
        }
    }

    fn input<T: FromStr>(prompt: &str) -> T
        where <T as FromStr>::Err: Debug
    {
        let type_name = std::any::type_name::<T>();
        println!("{} ({})", prompt, type_name.split_at(type_name.rfind("::").map_or(0, |i| i + 2)).1);

        loop {
            let mut in_str = String::new();
            std::io::stdin().read_line(&mut in_str).unwrap();
            let pred = ['\n', '\r'];
            let parsed = in_str.trim_matches(&pred[..]).parse();
            if parsed.is_err() {
                println!("Failed to parse. Please try again.");
            } else {
                break parsed.unwrap();
            }
        }
    }
}