use crate::{
    game::consts::{CELL_HEIGHT, CELL_WIDTH},
    structs::Vector2,
};

#[derive(Copy, Clone)]
pub struct Cell {
    pub width: i32,
    pub height: i32,
    pub vec: Vector2,
    pub cell_type: CellType,
}

impl Cell {
    pub unsafe fn new(vec: Vector2, cell_type: CellType) -> Self {
        Self {
            width: CELL_WIDTH,
            height: CELL_HEIGHT,
            vec,
            cell_type,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum CellType {
    Empty,
    Box,
    Player,
    Goal,
}
