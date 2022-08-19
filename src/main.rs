use std::env;
use std::fs;
use std::error::Error;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("More arguments required!");
        process::exit(1);
    }

    let (query, filename) =  parse_config(&args);
    let res = read_file(filename.to_string()).unwrap();
    let lines = search(query, &res);

    if lines.len() == 0 {
        println!("No Results found for '{}'", query);
        process::exit(1);
    }

    println!("{}", lines.join("\n"));
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filename = &args[2];

    return (query, filename);
}

fn search(term: &str, contents: &str) -> Vec<String> {
    let mut results: Vec<String> = Vec::new();
    let mut count = 0;

    for line in contents.lines() {
        if line.contains(term) {
            count = count + 1;
            let s = line.to_string();
            results.push(format!("{} {}", count, s));
        }
    }

    return results;
}

fn read_file(filename: String) -> Result<String, Box<dyn Error>> {
    let content = fs::read_to_string(filename)?;

    return Ok(content);
}
