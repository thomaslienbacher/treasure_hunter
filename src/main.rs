extern crate ansi_term;
extern crate clap;

mod game;

use game::*;
use clap::{Arg, App};

fn main() {
    let _ = ansi_term::enable_ansi_support();//ansi escape codes need to be enabled for windows

    let command_map: Vec<Command> = vec![
        Command::new("quit", || { std::process::exit(0) }),
        Command::new("q", || { std::process::exit(0) }),
    ];

    let matches = App::new("Treasure Hunter").version("0.1.0")
        .arg(Arg::with_name("size")
            .short("s")
            .long("size")
            .number_of_values(2)
            .multiple(true)
            .takes_value(true)
            .help("Size of the maze"))
        .arg(Arg::with_name("numtreasures")
            .short("t")
            .long("numtreasures")
            .takes_value(true)
            .help("Number of treasures"))
        .get_matches();

    let mut size = Vec2::new(29, 17);

    if let Some(mut values) = matches.values_of("size") {
        size.x = match values.next().unwrap().parse::<usize>() {
            Ok(x) => { x }
            Err(err) => {
                println!("Error: {}", err);
                println!("-s can only accept numbers");
                std::process::exit(1);
            }
        };
        size.y = match values.next().unwrap().parse::<usize>() {
            Ok(y) => { y }
            Err(err) => {
                println!("Error: {}", err);
                println!("-s can only accept numbers");
                std::process::exit(1);
            }
        };
    }

    let num_treasures = match matches.value_of("numtreasures").unwrap_or("5").parse::<i32>() {
        Ok(num_treasures) => { num_treasures }
        Err(err) => {
            println!("Error: {}", err);
            println!("-t can only accept numbers");
            std::process::exit(1);
        }
    };

    let mut map = Map::new(size, num_treasures);

    println!("\x1b[2J\x1b[H");

    loop {
        //display
        game::print_header();
        map.print();

        //update
        let input_raw = collect_input().to_lowercase();
        let input = input_raw.trim();

        map.move_player(input);//input gets validated inside functions

        println!("\x1b[2J\x1b[H");//clear screen

        for c in &command_map {
            if c.cmd.as_str().eq(input) {
                c.exec();
            }
        }
    }
}
