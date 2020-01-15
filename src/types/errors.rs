#[derive(Debug)]
pub enum TowerError {
    SmallerDiskOnTower,
    NoDisksOnTower,
}

#[derive(Debug)]
pub enum CursorError {
    AlreadyHoldingADisk { size: u8 },
    NoDisksToDrop,
    CannotDropDiskOnTower,
    TE(TowerError),
}
