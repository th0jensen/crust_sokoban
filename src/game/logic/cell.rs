use crate::structs::Vector2;

#[derive(Copy, Clone)]
pub struct Cell {
    pub vec: Vector2,
    pub base: CellBase,
    pub entity: CellEntity,
}

impl Cell {
    pub unsafe fn new(vec: Vector2, base: CellBase, entity: CellEntity) -> Self {
        Self { vec, base, entity }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum CellBase {
    Floor,
    Wall,
    Goal,
}

#[derive(Clone, Copy, PartialEq)]
pub enum CellEntity {
    None,
    Player,
    Box,
}
