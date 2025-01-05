// This file is used to test Zia, modifying this might result in broken tests.
// Here be the bears  ʕ•ᴥ•ʔ

// Zoo prints the map we are using, `BTReeMap` guarantees order, and thus makes
// the command deterministic
use std::collections::BTreeMap;

static USAGE: &str = "Usage: zoo <command> [options]
Commands:
  add: add [name] [nick]
  remove remove [name]
  list:list animals in the zoo
";

fn print_zoo(zoo: BTreeMap<String, String>) {
    if zoo.is_empty() {
        println!("The zoo is empty.");
    } else {
        println!("Animals in the zoo:");
        for (name, species) in &zoo {
            println!("{} the {}", name, species);
        }
    }
}
fn main() {
    let mut zoo = BTreeMap::from([
        ("Gator".to_string(), "The Alligator".to_string()),
        ("Melania".to_string(), "The Cute Bear".to_string()),
        ("Manny".to_string(), "The Monkey".to_string()),
    ]);

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprint!("{}", USAGE);
        std::process::exit(1);
    }

    match args[1].as_str() {
        "add" => {
            if args.len() != 4 {
                eprintln!("Usage: zoo add <name> <species>");
                std::process::exit(1);
            }
            let (name, species) = (args[2].clone(), args[3].clone());
            if zoo.contains_key(&name) {
                eprintln!("Error: Animal '{}' already exists.", name);
            } else {
                zoo.insert(name.clone(), species);
                println!("Added '{}' to the zoo.", name);
            }
            print_zoo(zoo);
        }

        "remove" => {
            if args.len() != 3 {
                eprintln!("Usage: zoo remove <name>");
                std::process::exit(1);
            }
            let name = &args[2];
            if zoo.remove(name).is_some() {
                println!("Removed '{}' from the zoo.", name);
            } else {
                eprintln!("Error: Animal '{}' not found.", name);
            }
            print_zoo(zoo);
        }

        "list" => print_zoo(zoo),

        _ => {
            eprintln!("Unknown command: {}", args[1]);
            eprintln!("Available commands: add, remove, list");
            std::process::exit(1);
        }
    }
}
