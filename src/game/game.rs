use crate::{
    ffi::libc::malloc,
    game::{
        consts::{COL_COUNT, ROW_COUNT},
        logic::{
            get_cells, Board, Cell,
            CellType::{Box, Empty, Player},
            Cells,
        },
    },
    structs::{append, Array, Vector2},
};

#[derive(Clone, Copy)]
pub struct Game {
    pub playing: bool,
    pub board: *mut Board,
}

impl Game {
    pub unsafe fn new() -> *mut Self {
        let game_ptr = malloc(size_of::<Game>());
        let board_ptr = malloc(size_of::<Board>());

        let start_pos = Vector2::new(2, 2);
        let mut boxes: Array<Vector2> = Array::new();

        generate_boxes(&mut boxes);

        *board_ptr = Board::new(ROW_COUNT, COL_COUNT, start_pos, boxes);
        *game_ptr = Self {
            playing: true,
            board: board_ptr,
        };

        game_ptr
    }

    pub unsafe fn destroy(_game: *mut Self) {}
}

pub unsafe fn generate_boxes(arr: *mut Array<Vector2>) -> *mut Array<Vector2> {
    append(arr, Vector2::new(1, 3));
    append(arr, Vector2::new(3, 2));
    append(arr, Vector2::new(3, 3));
    return arr;
}

pub unsafe fn get_board(game: *mut Game) -> *mut Board {
    (*game).board
}

pub unsafe fn get_cell(cells: Cells, x: i32, y: i32) -> *mut Cell {
    let row = &cells.items.add(y as usize).read();
    row.items.add(x as usize)
}

pub unsafe fn in_bounds(cells: Cells, x: i32, y: i32) -> bool {
    if x < 0 || y < 0 {
        return false;
    }

    if y as usize >= cells.count {
        return false;
    }

    let row = &cells.items.add(y as usize).read();

    if x as usize >= row.count {
        return false;
    }

    true
}

pub unsafe fn find_player(cells: Cells) -> Vector2 {
    let mut x = 0;
    let mut y = 0;

    for r in 0..cells.count {
        let row = &cells.items.add(r).read();
        for c in 0..row.count {
            let cell = row.items.add(c);
            if (*cell).cell_type != Player {
                continue;
            }
            (x, y) = (c as i32, r as i32);
        }
    }

    return Vector2::new(x, y);
}

pub unsafe fn try_move_player(cells: Cells, from: Vector2, to: Vector2) {
    let player = get_cell(cells, from.x, from.y);
    let target = get_cell(cells, to.x, to.y);

    (*player).cell_type = Empty;
    (*target).cell_type = Player;
}

pub unsafe fn try_push_box(
    cells: Cells,
    player_pos: Vector2,
    box_pos: Vector2,
    delta: Vector2,
) -> bool {
    let mut end = box_pos;

    loop {
        end = Vector2::new(end.x + delta.x, end.y + delta.y);

        if !in_bounds(cells, end.x, end.y) {
            return false;
        }

        let cell = get_cell(cells, end.x, end.y);

        if (*cell).cell_type == Empty {
            break;
        }

        if (*cell).cell_type != Box {
            return false;
        }
    }

    let player = get_cell(cells, player_pos.x, player_pos.y);
    let first_box = get_cell(cells, box_pos.x, box_pos.y);
    let empty_after_chain = get_cell(cells, end.x, end.y);

    (*empty_after_chain).cell_type = Box;
    (*first_box).cell_type = Player;
    (*player).cell_type = Empty;

    true
}

pub unsafe fn move_player(game: *mut Game, dir: Direction) {
    let cells = *get_cells(get_board(game));

    let player_pos = find_player(cells);
    let delta = Direction::delta(dir);

    let target_pos = Vector2::new(player_pos.x + delta.x, player_pos.y + delta.y);

    if !in_bounds(cells, target_pos.x, target_pos.y) {
        return;
    }

    let target = get_cell(cells, target_pos.x, target_pos.y);

    if (*target).cell_type == Empty {
        try_move_player(cells, player_pos, target_pos);
        return;
    }

    if (*target).cell_type == Box {
        try_push_box(cells, player_pos, target_pos, delta);
        return;
    }
}

pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    pub unsafe fn delta(dir: Direction) -> Vector2 {
        match dir {
            Direction::Left => Vector2::new(-1, 0),
            Direction::Right => Vector2::new(1, 0),
            Direction::Up => Vector2::new(0, -1),
            Direction::Down => Vector2::new(0, 1),
        }
    }
}
