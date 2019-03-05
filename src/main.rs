use clap::{App, Arg};

use crate::game::*;

mod game;

#[inline]
fn clear_screen() {
    println!("\x1b[2J\x1b[H");
}

fn main() {
    #[cfg(windows)] {
        let _ = ansi_term::enable_ansi_support();//ansi escape codes need to be enabled for windows
    }

    let matches = App::new("Treasure Hunter")
        .author("Thomas Lienbacher <lienbacher.tom@gmail.com>")
        .about("Small console game where you collect treasure in a maze")
        .version(env!("CARGO_PKG_VERSION"))
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

    clear_screen();

    loop {
        //display
        game::print_header();
        map.print();

        //update
        let input = collect_input().trim().to_lowercase();
        map.move_player(input.as_str());//input gets validated inside function
        clear_screen();

        match input.as_str() {
            "q" | "quit" => { std::process::exit(0) }
            _ => {}
        }
    }
}
