use crate::{
    game::consts::{CELL_HEIGHT, CELL_WIDTH},
    structs::Vector2,
};

#[derive(Copy, Clone)]
pub struct Cell {
    pub width: i32,
    pub height: i32,
    pub vec: Vector2,
    pub base: CellBase,
    pub entity: CellEntity,
}

impl Cell {
    pub unsafe fn new(vec: Vector2, base: CellBase, entity: CellEntity) -> Self {
        Self {
            width: CELL_WIDTH,
            height: CELL_HEIGHT,
            vec,
            base,
            entity,
        }
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
