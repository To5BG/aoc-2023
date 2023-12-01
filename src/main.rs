use clap::Parser;
use itertools::Itertools;
use std::{
    error::Error,
    fs::{read_dir, read_to_string},
};

mod solutions;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of times to greet
    #[arg(short, long, default_value_t = 0, help = "Which day to run")]
    day: u8,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let days = if args.day == 0 {
        read_dir("./src/solutions/")?
            .filter_map(|p| {
                p.ok()?
                    .path()
                    .file_stem()?
                    .to_str()
                    .map(|s| s[3..].to_string())
            })
            .sorted()
            .collect::<Vec<String>>()
    } else {
        Vec::from([args.day.to_string()])
    };
    for day in &days {
        let input = read_to_string("./inputs/".to_string() + day + ".in")
            .map_err(|_| "No input for given day found.")?;
        let (star_1, star_2, t) = solutions::solve(day, &input);
        println!(
            "Day {}:\n Part 1: {}\n Part 2: {}\n Time  : {:?}\n",
            day, star_1, star_2, t
        );
    }
    Ok(())
}
