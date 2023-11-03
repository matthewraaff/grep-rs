use std::env;
use std::fs::File;
use std::io::{self, BufRead};

use regex::Regex;
use console::style;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let types = ["file", "dir"];
    let _ignore = ["exe", ".dll", ".pdb"]; // TODO: implement

    if args.len() != 4 {
        println!("Usage: {{}} <type> <pattern> <file>");
        println!("Type must be one of: {}", style(types.join(", ")).red());
        return Ok(());
    }

    let type_ = &args[1];
    let pattern = &args[2];
    let file = &args[3];
    let re = Regex::new(pattern).unwrap();
    
    if !types.contains(&type_.as_str()) {
        println!("Type must be one of: {}", style(types.join(", ")).red());
        return Ok(());
    }

    if type_ == "file" {
        return search_file(file, &re);
    }

    if type_ == "dir" {
        return search_dir(file, &re);
    }

    Ok(())
}

fn search_file(file: &str, re: &Regex) -> io::Result<()> {
    let f = File::open(file)?;
    let reader = io::BufReader::new(f);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        if re.is_match(&line) {
            println!("{}: {}", index, line);
        }
    }

    Ok(())
}

fn search_dir(dir: &str, re: &Regex) -> io::Result<()> {
    let paths = std::fs::read_dir(dir)?;

    for path in paths {
        let path = path.unwrap().path();
        if path.is_dir() {
            search_dir(path.to_str().unwrap(), re)?;
        } else {
            search_file(path.to_str().unwrap(), re)?;
        }
    }

    Ok(())
}