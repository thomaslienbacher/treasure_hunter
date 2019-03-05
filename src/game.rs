use std::collections::LinkedList;
use std::io;
use std::io::*;

use ansi_term::Colour::*;
use rand::prelude::*;

pub fn print_header() {
    println!("Treasure Hunter!\n");
    println!("Use -h to get extra help");
    println!("Use w, a, s, d to move");
    println!("Use q to quit");
    println!("Collect all the treasure\n");
}

pub fn collect_input() -> String {
    print!("> ");
    io::stdout().flush().expect("Couldn't flush stdout");

    let mut input = String::new();
    stdin().read_line(&mut input).expect("Couldn't get input from stdin");

    input
}


#[derive(Copy, Clone, Debug)]
pub struct Vec2 {
    pub x: usize,
    pub y: usize,
}

impl Vec2 {
    pub fn new(x: usize, y: usize) -> Vec2 {
        Vec2 { x, y }
    }
}

pub struct Map {
    map: Vec<Vec<char>>,
    player: Vec2,
    treasure_found: i32,
}

impl Map {
    pub fn new(mut size: Vec2, num_treasures: i32) -> Map {
        if size.x % 2 == 0 {
            size.x += 1;
        }
        if size.y % 2 == 0 {
            size.y += 1;
        }


        let tx = num::clamp(size.x, 5, 377);
        let ty = num::clamp(size.y, 5, 377);

        size.x = ty;//values are swapped
        size.y = tx;

        let mut m = Map {
            map: vec![vec!['#'; size.y]; size.x],
            player: Vec2::new(1, 1),
            treasure_found: 0,
        };

        //generate maze
        let mut stack: LinkedList<Vec2> = LinkedList::new();
        let mut nbrs: [i32; 4];//top, right, bottom, left neighbors
        let mut cur = Vec2::new(1, 1);

        loop {
            nbrs = [0; 4];

            if cur.y < size.y - 3 { nbrs[0] = (m.map[cur.x][cur.y + 2] == '#') as i32 }
            if cur.x < size.x - 3 { nbrs[1] = (m.map[cur.x + 2][cur.y] == '#') as i32 }
            if cur.y > 2 { nbrs[2] = (m.map[cur.x][cur.y - 2] == '#') as i32 }
            if cur.x > 2 { nbrs[3] = (m.map[cur.x - 2][cur.y] == '#') as i32 }

            //has nbrs
            if (nbrs[0] | nbrs[1] | nbrs[2] | nbrs[3]) != 0 {
                let mut r = thread_rng().gen_range(0, 4);
                while nbrs[r] == 0 {
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
        for _ in 0..num_treasures {
            let mut x = thread_rng().gen_range(0, size.x);
            let mut y = thread_rng().gen_range(0, size.y);

            while m.map[x][y] != ' ' {
                x = thread_rng().gen_range(0, size.x);
                y = thread_rng().gen_range(0, size.y);
            }

            m.map[x][y] = 'T';
        }

        m
    }

    pub fn print(&mut self) {
        self.map[self.player.x][self.player.y] = 'X';

        println!("Treasure found: {}\n", self.treasure_found);

        for x in 0..self.map.len() {
            for y in 0..self.map[0].len() {
                let s = self.map[x][y].to_string();

                match self.map[x][y] {
                    '#' => { print!("{}", White.paint(s)) }
                    'T' => { print!("{}", Yellow.paint(s)) }
                    'X' => { print!("{}", Red.paint(s)) }
                    _ => { print!("{}", self.map[x][y]) }
                }
            }

            println!();
        }

        println!();
    }

    pub fn move_player(&mut self, input: &str) {
        for c in input.chars() {
            match c {
                'w' | 's' | 'a' | 'd' => {}
                _ => { return; }
            }
        }

        for c in input.chars() {
            self.map[self.player.x][self.player.y] = ' ';

            match c {
                'w' => {
                    if self.player.x > 0 && self.map[self.player.x - 1][self.player.y] != '#' {
                        self.player.x -= 1
                    }
                }
                's' => {
                    if self.player.x < self.map.len() - 1 && self.map[self.player.x + 1][self.player.y] != '#' {
                        self.player.x += 1
                    }
                }
                'a' => {
                    if self.player.y > 0 && self.map[self.player.x][self.player.y - 1] != '#' {
                        self.player.y -= 1
                    }
                }
                'd' => {
                    if self.player.y < self.map[0].len() - 1 && self.map[self.player.x][self.player.y + 1] != '#' {
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
