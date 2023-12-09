use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::collections::{HashMap, VecDeque};

fn main() -> io::Result<()> {
    let file = File::open("big_text.log")?;
    let reader = BufReader::new(file);

    let x = 5; // Change x to the desired number of lines to read from the beginning and end.

    let mut first_lines = Vec::new();
    let mut last_lines = VecDeque::new();

    for (index, line) in reader.lines().enumerate() {
        let line = line?;
        if index < x {
            first_lines.push(line.clone());
        }
        last_lines.push_back(line.clone());

        // Keep only the last x lines in the last_lines queue.
        while last_lines.len() > x {
            last_lines.pop_front();
        }
    }

    // You can store the lines in a Vec or HashMap here, depending on your needs.
    let mut lines_map = HashMap::new();
    lines_map.insert("first_lines", first_lines);
    lines_map.insert("last_lines", last_lines.into_iter().collect::<Vec<String>>());

    // Example: Print the first and last lines
    println!("First {} lines:", x);
    for line in lines_map.get("first_lines").unwrap() {
        println!("{}", line);
    }

    println!("Last {} lines:", x);
    for line in lines_map.get("last_lines").unwrap() {
        println!("{}", line);
    }

    Ok(())
}
