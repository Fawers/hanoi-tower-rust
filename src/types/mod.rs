// mod drawable;
pub mod errors;

use errors::{CursorError as ce, TowerError as te};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct Disk {
    size: u8,
}

impl Disk {
    pub fn new(size: u8) -> Self {
        match size {
            1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 => Disk { size },
            _ => panic!("Invalid disk size. size > 8 || size == 0"),
        }
    }

    pub fn size(&self) -> u8 {
        self.size
    }
}

#[derive(Debug)]
pub struct Tower {
    id: u8,
    pub disks: Vec<Disk>,
}

impl Tower {
    pub fn new() -> Self {
        static mut ID: u8 = 0;

        unsafe {
            ID += 1;
            Tower {
                id: ID,
                disks: vec![],
            }
        }
    }

    pub fn as_rc(self) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(self))
    }

    pub fn id(&self) -> u8 {
        self.id
    }

    pub fn push(&mut self, disk: Disk) -> Result<(), (errors::TowerError, Disk)> {
        if self.can_fit(&disk) {
            self.disks.push(disk);
            Ok(())
        } else {
            Err((te::SmallerDiskOnTower, disk))
        }
    }

    pub fn pop(&mut self) -> Result<Disk, errors::TowerError> {
        match self.disks.pop() {
            Some(disk) => Ok(disk),
            None => Err(te::NoDisksOnTower),
        }
    }

    pub fn can_fit(&self, disk: &Disk) -> bool {
        match self.peek() {
            Some(d) => d.size > disk.size,
            None => true,
        }
    }

    pub fn peek(&self) -> Option<&Disk> {
        self.disks.last()
    }
}

#[derive(Debug)]
pub struct Cursor {
    pub selected_tower: Rc<RefCell<Tower>>,
    pub disk: Option<Disk>,
}

impl Cursor {
    pub fn take(&mut self) -> Result<(), errors::CursorError> {
        match self.disk {
            Some(ref disk) => Err(ce::AlreadyHoldingADisk { size: disk.size }),
            None => match self.selected_tower.borrow_mut().pop() {
                Ok(disk) => {
                    self.disk = Some(disk);
                    Ok(())
                }
                Err(te) => Err(ce::TE(te)),
            },
        }
    }

    pub fn drop(&mut self) -> Result<(), errors::CursorError> {
        if let Some(disk) = self.disk.take() {
            match self.selected_tower.borrow_mut().push(disk) {
                Ok(()) => Ok(()),
                Err((te::SmallerDiskOnTower, disk)) => {
                    self.disk.replace(disk);
                    Err(ce::CannotDropDiskOnTower)
                }
                Err((te, _)) => Err(ce::TE(te)),
            }
        } else {
            Err(ce::NoDisksToDrop)
        }
    }
}
