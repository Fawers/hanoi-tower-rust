pub mod types;

use std::cell::RefCell;
use std::rc::Rc;
use types::errors::{CursorError as ce, TowerError as te};
use types::{Cursor, Disk, Tower};

#[derive(Debug)]
pub struct Game {
    pub towers: [Rc<RefCell<Tower>>; 3],
    pub cursor: Cursor,
    moves: u32,
    disks: u8,
}

impl Game {
    pub fn new(disks: u8) -> Self {
        let towers = [
            Tower::new().as_rc(),
            Tower::new().as_rc(),
            Tower::new().as_rc(),
        ];

        let cursor = Cursor {
            disk: None,
            selected_tower: towers[0].clone(),
        };

        {
            let mut t = towers[0].borrow_mut();
            for i in (1..=disks).rev() {
                t.push(Disk::new(i)).unwrap(); // tower is empty, can't error
            }
        }

        Game {
            towers,
            cursor,
            disks,
            moves: 0,
        }
    }

    pub fn run(&mut self) {
        println!("Use the numbers 2, 4, 6, and 8 (preferably on the numpad)");
        println!("to play. 4 and 6 move left or right, respectively,");
        println!("8 takes a disk from the current tower, and 2 drops the");
        println!("disk on it. Type q to quit.\n");

        self.main_loop();
        println!("Total number of moves: {}", self.moves,);

        if self.solved() {
            println!(
                "You solved the puzzle{}! Yay!",
                match self.moves == 2u32.pow(self.disks as u32) - 1 {
                    true => " with the optimal solution",
                    _ => "",
                }
            );
        } else {
            println!("You didn't solve the puzzle. :(");
        }
    }

    fn solved(&self) -> bool {
        self.towers[2].borrow().disks.len() as u8 == self.disks
    }

    fn main_loop(&mut self) {
        let mut buff = String::new();
        let stdin = std::io::stdin();

        while !self.solved() {
            self.draw_everything();
            stdin.read_line(&mut buff).expect("Can't read from stdin");

            match buff.chars().nth(0) {
                Some('q') => break,
                Some('2') => match self.cursor.drop() {
                    Ok(()) => {
                        println!("Dropped disk on tower.");
                        self.moves += 1;
                    }
                    Err(ce::CannotDropDiskOnTower) => {
                        println!("Cannot drop this disk on this tower.")
                    }
                    Err(ce::NoDisksToDrop) => println!("Not holding any disks now."),
                    Err(ce::TE(te::SmallerDiskOnTower)) => println!("Disk on tower is smaller."),
                    Err(e) => println!("Error while dropping: {:?}", e),
                },
                Some('8') => match self.cursor.take() {
                    Ok(()) => println!("Took disk from tower."),
                    Err(ce::AlreadyHoldingADisk { .. }) => println!("Already holding a disk."),
                    Err(ce::TE(te::NoDisksOnTower)) => println!("Tower is empty."),
                    Err(e) => println!("Error while taking: {:?}", e),
                },
                Some('4') => {
                    let t = self.cursor.selected_tower.borrow().id() as usize;
                    match t {
                        1 => println!("Can't move cursor to the left."),
                        _ => {
                            self.cursor.selected_tower = self.towers[t - 2].clone();
                            println!("Moved to the tower on the left.");
                        }
                    }
                }
                Some('6') => {
                    let t = self.cursor.selected_tower.borrow().id() as usize;
                    match t {
                        3 => println!("Can't move cursor to the right."),
                        _ => {
                            self.cursor.selected_tower = self.towers[t].clone();
                            println!("Moved to the tower on the right.");
                        }
                    }
                }
                Some(c) => println!("Unrecognized character: {:?}.", c),
                None => println!("Input is empty."),
            }

            buff.clear();
        }
    }

    fn draw_everything(&self) {
        for t in &self.towers {
            println!(
                "Disks on tower #{}: {:?}",
                t.borrow().id(),
                t.borrow().disks
            );
        }

        print!(
            "Cursor is over tower #{}, and it's ",
            self.cursor.selected_tower.borrow().id(),
        );

        match &self.cursor.disk {
            Some(d) => println!("holding a disk of size {}.\n", d.size()),
            None => println!("not holding anything.\n"),
        };
    }
}
