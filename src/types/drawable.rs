use super::{Cursor, Disk, Tower};

const TOWER: &str = "|";
const TOWER_BASE: &str = "_";
const DISK: &str = "█";
const CURSOR: &str = "▼";

pub trait Drawable {
    fn as_lines(&self) -> Vec<String>;
    fn draw(&self) {
        println!("{}", self.as_lines().join("\n"));
    }
}

impl Drawable for Cursor {
    fn as_lines(&self) -> Vec<String> {
        vec![CURSOR.to_owned()]
    }
}

impl Drawable for Disk {
    fn as_lines(&self) -> Vec<String> {
        vec![vec![DISK; self.size as usize].join("")]
    }
}

impl Drawable for Tower {
    fn as_lines(&self) -> Vec<String> {
        let size = 9;
        let mut foundation = String::new();

        match self.peek() {
            Some(_d) => (),
            None => ()
        };

        vec![]
    }
}
