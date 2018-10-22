extern crate rand;
extern crate ansi_term;

use std::io;
use std::io::*;
use std::boxed::*;
use std::collections::LinkedList;

use self::rand::prelude::*;
use ansi_term::Colour::*;


//constants
const SIZE: Vec2 = Vec2 { x: 17, y: 29 };
const NUM_TREASURE: usize = 5;


pub fn print_header() {
    println!("Welcome to Treasure Hunter!\n");
    println!("Use w, a, s, d to move");
    println!("Use q to quit");
    println!("Collect all the treasure\n");
}

pub fn collect_input() -> String {
    print!("> ");
    io::stdout().flush().expect("Could't flush stdout");

    let mut input = String::new();

    stdin().read_line(&mut input).expect("Couldn't get input from stdin");

    input
}


#[derive(Copy, Clone, Debug)]
pub struct Vec2 {
    x: usize,
    y: usize,
}

impl Vec2 {
    pub fn new(x: usize, y: usize) -> Vec2 {
        Vec2 { x, y }
    }
}


pub struct Command {
    pub cmd: String,
    func: Box<Fn()>,
}

impl Command {
    pub fn new(cmd: &'static str, func: fn()) -> Command {
        Command { cmd: String::from(cmd), func: Box::new(func) }
    }

    pub fn exec(&self) {
        (self.func)();
    }
}


//TODO: implement dynamic sizing
pub struct Map {
    map: [[char; SIZE.y]; SIZE.x],
    player: Vec2,
    treasure_found: i32,
}

impl Map {
    pub fn new() -> Map {
        let mut m = Map {
            map: [['#'; SIZE.y]; SIZE.x],
            player: Vec2::new(1, 1),
            treasure_found: 0,
        };

        //generate maze
        let mut stack: LinkedList<Vec2> = LinkedList::new();
        let mut nbr: [i32; 4];//top, right, bottom, left
        let mut cur = Vec2::new(1, 1);

        loop {
            nbr = [0; 4];

            if cur.y < SIZE.y - 3 { nbr[0] = (m.map[cur.x][cur.y + 2] == '#') as i32 }
            if cur.x < SIZE.x - 3 { nbr[1] = (m.map[cur.x + 2][cur.y] == '#') as i32 }
            if cur.y > 2 { nbr[2] = (m.map[cur.x][cur.y - 2] == '#') as i32 }
            if cur.x > 2 { nbr[3] = (m.map[cur.x - 2][cur.y] == '#') as i32 }


            //has nbrs
            if (nbr[0] | nbr[1] | nbr[2] | nbr[3]) != 0 {
                let mut r = thread_rng().gen_range(0, 4);
                while nbr[r] == 0 {
                    r = thread_rng().gen_range(0, 4)
                }

                if r == 0 {
                    m.map[cur.x][cur.y] = ' ';
                    m.map[cur.x][cur.y + 1] = ' ';
                    m.map[cur.x][cur.y + 2] = ' ';

                    cur = Vec2::new(cur.x, cur.y + 2);
                    stack.push_back(cur);
                    continue;
                }

                if r == 1 {
                    m.map[cur.x][cur.y] = ' ';
                    m.map[cur.x + 1][cur.y] = ' ';
                    m.map[cur.x + 2][cur.y] = ' ';

                    cur = Vec2::new(cur.x + 2, cur.y);
                    stack.push_back(cur);
                    continue;
                }

                if r == 2 {
                    m.map[cur.x][cur.y] = ' ';
                    m.map[cur.x][cur.y - 1] = ' ';
                    m.map[cur.x][cur.y - 2] = ' ';

                    cur = Vec2::new(cur.x, cur.y - 2);
                    stack.push_back(cur);
                    continue;
                }

                if r == 3 {
                    m.map[cur.x][cur.y] = ' ';
                    m.map[cur.x - 1][cur.y] = ' ';
                    m.map[cur.x - 2][cur.y] = ' ';

                    cur = Vec2::new(cur.x - 2, cur.y);
                    stack.push_back(cur);
                    continue;
                }
            } else {
                cur = match stack.pop_back() {
                    Some(cur) => { cur }
                    None => { break; }//stack is empty
                };
            }
        }

        //place treasures
        for _ in 0..NUM_TREASURE {
            let mut x = thread_rng().gen_range(0, SIZE.x);
            let mut y = thread_rng().gen_range(0, SIZE.y);

            while m.map[x][y] == '#' {
                x = thread_rng().gen_range(0, SIZE.x);
                y = thread_rng().gen_range(0, SIZE.y);
            }

            m.map[x][y] = 'T';
        }

        m
    }

    pub fn print(&mut self) {
        self.map[self.player.x][self.player.y] = 'X';

        println!("Treasure found: {}\n", self.treasure_found);

        for x in 0..SIZE.x {
            for y in 0..SIZE.y {
                let s = self.map[x][y].to_string();

                match self.map[x][y] {
                    '#' => {print!("{}", White.paint(s))}
                    'T' => {print!("{}", Yellow.paint(s))}
                    'X' => {print!("{}", Red.paint(s))}
                    _ => { print!("{}", self.map[x][y]) }
                }
            }

            println!();
        }

        println!();
    }

    pub fn move_player(&mut self, input: &str) {
        fn valid_input(input: &str) -> bool {
            let mut valid = true;

            for c in input.chars() {
                match c {
                    'w' => {}
                    's' => {}
                    'a' => {}
                    'd' => {}
                    _ => { valid = false }
                }
            }

            valid
        }

        if !valid_input(input) {
            return;
        }

        for c in input.chars() {
            self.map[self.player.x][self.player.y] = ' ';

            match c { //TODO: fix x and y swap
                'w' => {
                    if self.player.x > 0 && self.map[self.player.x - 1][self.player.y] != '#' {
                        self.player.x -= 1
                    }
                }
                's' => {
                    if self.player.x < SIZE.x - 1 && self.map[self.player.x + 1][self.player.y] != '#' {
                        self.player.x += 1
                    }
                }
                'a' => {
                    if self.player.y > 0 && self.map[self.player.x][self.player.y - 1] != '#' {
                        self.player.y -= 1
                    }
                }
                'd' => {
                    if self.player.y < SIZE.y - 1 && self.map[self.player.x][self.player.y + 1] != '#' {
                        self.player.y += 1
                    }
                }
                _ => {}
            }

            if self.map[self.player.x][self.player.y] == 'T' {
                self.treasure_found += 1;
            }
        }
    }
}
