use crate::{
    ffi::libc::{free, malloc},
    game::logic::{
        get_cells, Board, Cell,
        CellBase::{Goal, Wall},
        CellEntity::{Box, None, Player},
        Cells,
    },
    structs::{append, Array, Vector2},
};

#[derive(Clone, Copy)]
pub struct Game {
    pub score: i32,
    pub top_score: i32,
    pub playing: bool,
    pub board: *mut Board,
}

impl Game {
    pub unsafe fn new() -> *mut Self {
        let game_ptr = malloc(size_of::<Game>());
        let board_ptr = malloc(size_of::<Board>());

        let start_pos = Vector2::new(2, 2);
        let mut boxes: Array<Vector2> = Array::new();
        let mut goals: Array<Vector2> = Array::new();
        let mut walls: Array<Vector2> = Array::new();

        generate_boxes(&mut boxes);
        generate_goals(&mut goals);
        generate_walls(&mut walls);

        *board_ptr = Board::new(5, 5, start_pos, boxes, goals, walls);
        *game_ptr = Self {
            score: 0,
            top_score: goals.count as i32,
            playing: true,
            board: board_ptr,
        };

        game_ptr
    }

    pub unsafe fn destroy(game: *mut Self) {
        if game.is_null() {
            return;
        }

        if !(*game).board.is_null() {
            Board::destroy((*game).board);
            free((*game).board);
            (*game).board = core::ptr::null_mut();
        }

        free(game);
    }
}

pub unsafe fn generate_boxes(arr: *mut Array<Vector2>) -> *mut Array<Vector2> {
    append(arr, Vector2::new(1, 3));
    append(arr, Vector2::new(3, 2));
    append(arr, Vector2::new(3, 3));
    return arr;
}

pub unsafe fn generate_goals(arr: *mut Array<Vector2>) -> *mut Array<Vector2> {
    append(arr, Vector2::new(0, 2));
    append(arr, Vector2::new(3, 4));
    append(arr, Vector2::new(4, 3));
    return arr;
}

pub unsafe fn generate_walls(arr: *mut Array<Vector2>) -> *mut Array<Vector2> {
    append(arr, Vector2::new(0, 1));
    append(arr, Vector2::new(0, 3));
    append(arr, Vector2::new(0, 0));
    return arr;
}

pub unsafe fn get_board(game: *mut Game) -> *mut Board {
    (*game).board
}

pub unsafe fn get_cell(cells: Cells, vec: Vector2) -> *mut Cell {
    let row = &cells.items.add(vec.y as usize).read();
    row.items.add(vec.x as usize)
}

pub unsafe fn in_bounds(cells: Cells, vec: Vector2) -> bool {
    if vec.x < 0 || vec.y < 0 {
        return false;
    }

    if vec.y as usize >= cells.count {
        return false;
    }

    let row = &cells.items.add(vec.y as usize).read();

    if vec.x as usize >= row.count {
        return false;
    }

    let target = get_cell(cells, vec);
    if (*target).base == Wall {
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
            if (*cell).entity != Player {
                continue;
            }
            (x, y) = (c as i32, r as i32);
        }
    }

    return Vector2::new(x, y);
}

pub unsafe fn try_move_player(cells: Cells, from: Vector2, to: Vector2) {
    let player = get_cell(cells, from);
    let target = get_cell(cells, to);

    (*player).entity = None;
    (*target).entity = Player;
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

        if !in_bounds(cells, end) {
            return false;
        }

        let cell = get_cell(cells, end);

        if (*cell).entity == None {
            break;
        }

        if (*cell).entity != Box {
            return false;
        }
    }

    let player = get_cell(cells, player_pos);
    let first_box = get_cell(cells, box_pos);
    let empty_after_chain = get_cell(cells, end);

    (*empty_after_chain).entity = Box;
    (*first_box).entity = Player;
    (*player).entity = None;

    true
}

pub unsafe fn move_player(game: *mut Game, dir: Direction) {
    let cells = *get_cells(get_board(game));

    let player_pos = find_player(cells);
    let delta = Direction::delta(dir);

    let target_pos = Vector2::new(player_pos.x + delta.x, player_pos.y + delta.y);

    if !in_bounds(cells, target_pos) {
        return;
    }

    let target = get_cell(cells, target_pos);

    if (*target).entity == None {
        try_move_player(cells, player_pos, target_pos);
        return;
    }

    if (*target).entity == Box {
        if try_push_box(cells, player_pos, target_pos, delta) {
            (*game).score = check_score(cells);
        };
        if (*game).score == (*game).top_score {
            (*game).playing = false;
        }
        return;
    }
}

unsafe fn check_score(cells: Cells) -> i32 {
    let mut score = 0;
    for r in 0..cells.count {
        let row = &cells.items.add(r).read();
        for c in 0..row.count {
            let cell = row.items.add(c);
            if (*cell).entity == Box && (*cell).base == Goal {
                score += 1;
                continue;
            }
        }
    }
    return score;
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
