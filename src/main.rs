extern crate ansi_term;

mod game;

use game::*;

fn main() {
    ansi_term::enable_ansi_support();//ansi escape codes need to be enabled for windows

    let command_map: Vec<Command> = vec![
        Command::new("quit", || { std::process::exit(0) }),
        Command::new("q", || { std::process::exit(0) }),
    ];

    let mut map = Map::new();

    loop {
        //display
        game::print_header();
        map.print();

        //update
        let input_raw = collect_input().to_lowercase();
        let input= input_raw.trim();

        map.move_player(input);//input gets validated inside functions

        println!("\x1b[2J\x1b[H");//clear screen

        for c in &command_map {
            if c.cmd.as_str().eq(input) {
                c.exec();
            }
        }
    }
}
