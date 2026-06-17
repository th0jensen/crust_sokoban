use crate::{
    ffi::libc::{free, malloc, rand, strcmp},
    game::{
        logic::{
            get_cells, Board, Cell,
            CellBase::{Goal, Wall},
            CellEntity::{Box, None, Player},
            Cells,
        },
        Difficulty::Easy,
    },
    structs::{append, pop, Array, Vector2},
};

#[derive(Clone, Copy, PartialEq)]
pub enum Difficulty {
    Easy = 5,
    Medium = 7,
    Hard = 9,
    Impossible = 11,
}

impl Difficulty {
    pub unsafe fn from(input: *const i8) -> Self {
        if strcmp(input, c"easy".as_ptr()) == 0 {
            return Difficulty::Easy;
        }
        if strcmp(input, c"medium".as_ptr()) == 0 {
            return Difficulty::Medium;
        }
        if strcmp(input, c"hard".as_ptr()) == 0 {
            return Difficulty::Hard;
        }
        if strcmp(input, c"impossible".as_ptr()) == 0 {
            return Difficulty::Impossible;
        }

        Difficulty::Easy
    }
}

#[derive(Clone, Copy)]
pub struct Game {
    pub score: i32,
    pub top_score: i32,
    pub playing: bool,
    pub board: *mut Board,
    pub undo_stack: *mut UndoStack,
}

impl Game {
    pub unsafe fn new(diff: Difficulty) -> *mut Self {
        let game_ptr = malloc(size_of::<Game>());
        let board_ptr = malloc(size_of::<Board>());

        let (player, mut boxes, mut goals, mut walls) = gen_board(diff);
        *board_ptr = Board::new(diff as i32, diff as i32, player, boxes, goals, walls);
        *game_ptr = Self {
            score: 0,
            top_score: goals.count as i32,
            playing: true,
            board: board_ptr,
            undo_stack: UndoStack::new(),
        };

        Array::destroy(&mut boxes);
        Array::destroy(&mut goals);
        Array::destroy(&mut walls);

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

        if !(*game).undo_stack.is_null() {
            let undo_stack = (*game).undo_stack;
            Array::destroy(&mut (*undo_stack).stack);
            free(undo_stack);
            (*game).undo_stack = core::ptr::null_mut();
        }

        free(game);
    }
}

pub unsafe fn gen_board(
    diff: Difficulty,
) -> (Vector2, Array<Vector2>, Array<Vector2>, Array<Vector2>) {
    let size = diff as i32;
    let box_count = (size + 1) / 2;
    let wall_count = if diff == Easy { (size + 1) / 2 } else { size };

    let (mut boxes, mut goals, mut walls, mut pool) =
        (Array::new(), Array::new(), Array::new(), Array::new());

    for y in 0..size {
        for x in 0..size {
            append(&mut pool, Vector2::new(x, y));
        }
    }

    let player = pick_weighted(&mut pool, size, flat_weight);

    for _ in 0..wall_count {
        append(&mut walls, pick_weighted(&mut pool, size, edge_weight));
    }

    for _ in 0..box_count {
        append(&mut boxes, pick_weighted(&mut pool, size, center_weight));
    }

    for _ in 0..box_count {
        append(&mut goals, pick_weighted(&mut pool, size, edge_weight));
    }

    Array::destroy(&mut pool);

    (player, boxes, goals, walls)
}

unsafe fn random_index(max: usize) -> usize {
    (rand() as usize) % max
}

unsafe fn clamp(value: i32, min: i32, max: i32) -> i32 {
    return if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    };
}

unsafe fn pick_weighted(
    pool: *mut Array<Vector2>,
    size: i32,
    weight_fn: unsafe fn(Vector2, i32) -> i32,
) -> Vector2 {
    let mut total_weight = 0;

    for i in 0..(*pool).count {
        let pos = *(*pool).items.add(i);
        total_weight += clamp(weight_fn(pos, size), 0, size * size);
    }

    let mut roll = rand() % total_weight;

    for i in 0..(*pool).count {
        let pos = *(*pool).items.add(i);
        let weight = clamp(weight_fn(pos, size), 0, size * size);

        if roll < weight {
            let picked = pos;

            let last_index = (*pool).count - 1;
            *(*pool).items.add(i) = *(*pool).items.add(last_index);
            (*pool).count -= 1;

            return picked;
        }

        roll -= weight;
    }

    let picked = *(*pool).items.add(0);
    (*pool).count -= 1;
    picked
}

unsafe fn flat_weight(_pos: Vector2, _size: i32) -> i32 {
    1
}

unsafe fn edge_weight(pos: Vector2, size: i32) -> i32 {
    let left = pos.x;
    let right = size - 1 - pos.x;
    let top = pos.y;
    let bot = size - 1 - pos.y;

    let mut edge_dist = left;
    if right < edge_dist {
        edge_dist = right;
    };
    if top < edge_dist {
        edge_dist = top;
    };
    if bot < edge_dist {
        edge_dist = bot;
    };

    let w = size - edge_dist;
    w * w
}

unsafe fn center_weight(pos: Vector2, size: i32) -> i32 {
    let center = size / 2;

    if pos.x == 0 || pos.y == 0 || pos.x == size - 1 || pos.y == size - 1 {
        return 0;
    }

    let dx = if pos.x > center {
        pos.x - center
    } else {
        center - pos.x
    };
    let dy = if pos.y > center {
        pos.y - center
    } else {
        center - pos.y
    };

    let dist = dx + dy;
    let w = size - dist;
    w * w
}

unsafe fn shuffle(arr: *mut Array<Vector2>) {
    if (*arr).count < 2 {
        return;
    }

    for i in 0..(*arr).count {
        let remaining = (*arr).count - i;
        let j = i + random_index(remaining);

        let a = (*arr).items.add(i);
        let b = (*arr).items.add(j);

        let tmp = *a;
        *a = *b;
        *b = tmp;
    }
}

pub unsafe fn gen_boxes(arr: *mut Array<Vector2>) -> *mut Array<Vector2> {
    append(arr, Vector2::new(1, 3));
    append(arr, Vector2::new(3, 2));
    append(arr, Vector2::new(3, 3));
    return arr;
}

pub unsafe fn gen_goals(arr: *mut Array<Vector2>) -> *mut Array<Vector2> {
    append(arr, Vector2::new(0, 2));
    append(arr, Vector2::new(3, 4));
    append(arr, Vector2::new(4, 3));
    return arr;
}

pub unsafe fn gen_walls(arr: *mut Array<Vector2>) -> *mut Array<Vector2> {
    append(arr, Vector2::new(0, 1));
    append(arr, Vector2::new(0, 3));
    append(arr, Vector2::new(0, 0));
    return arr;
}

pub unsafe fn get_board(game: *mut Game) -> *mut Board {
    (*game).board
}

pub unsafe fn get_undo_stack(game: *mut Game) -> *mut UndoStack {
    (*game).undo_stack
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
) -> (bool, Option<Vector2>) {
    let mut end = box_pos;

    loop {
        end = Vector2::new(end.x + delta.x, end.y + delta.y);

        if !in_bounds(cells, end) {
            return (false, Option::None);
        }

        let cell = get_cell(cells, end);

        if (*cell).entity == None {
            break;
        }

        if (*cell).entity != Box {
            return (false, Option::None);
        }
    }

    let player = get_cell(cells, player_pos);
    let first_box = get_cell(cells, box_pos);
    let empty_after_chain = get_cell(cells, end);

    (*empty_after_chain).entity = Box;
    (*first_box).entity = Player;
    (*player).entity = None;

    (true, Some(end))
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
        UndoStack::push(
            (*game).undo_stack,
            PlayerMove {
                delta,
                pushed: false,
                box_to: Vector2::new(0, 0),
            },
        );

        return;
    }

    if (*target).entity == Box {
        let (ok, end_pos) = try_push_box(cells, player_pos, target_pos, delta);
        if ok {
            let box_to = match end_pos {
                Some(pos) => pos,
                Option::None => return,
            };

            (*game).score = check_score(cells);
            UndoStack::push(
                (*game).undo_stack,
                PlayerMove {
                    delta,
                    pushed: true,
                    box_to,
                },
            );
        };

        if (*game).score == (*game).top_score {
            (*game).playing = false;
        }
        return;
    }
}

#[derive(Clone, Copy)]
pub struct UndoStack {
    pub stack: Array<PlayerMove>,
}

impl UndoStack {
    pub unsafe fn new() -> *mut Self {
        let undo_stack_ptr = malloc(size_of::<UndoStack>());
        let arr: Array<PlayerMove> = Array::new();
        *undo_stack_ptr = Self { stack: arr };

        undo_stack_ptr
    }

    pub unsafe fn push(undo_stack: *mut UndoStack, mv: PlayerMove) {
        append(&mut (*undo_stack).stack, mv);
    }

    pub unsafe fn pop(undo_stack: *mut UndoStack) -> PlayerMove {
        pop(&mut (*undo_stack).stack)
    }
}

#[derive(Clone, Copy)]
pub struct PlayerMove {
    delta: Vector2,
    pushed: bool,
    box_to: Vector2,
}

pub unsafe fn undo_move(game: *mut Game) {
    if (*get_undo_stack(game)).stack.count < 1 {
        return;
    }

    let latest_move = UndoStack::pop(get_undo_stack(game));
    let cells = *get_cells(get_board(game));
    let player = find_player(cells);
    let prev_player = Vector2::new(
        player.x - latest_move.delta.x,
        player.y - latest_move.delta.y,
    );

    let curr = get_cell(cells, player);
    let prev = get_cell(cells, prev_player);

    if !latest_move.pushed {
        (*prev).entity = Player;
        (*curr).entity = None;
    } else {
        let box_to = get_cell(cells, latest_move.box_to);
        (*prev).entity = Player;
        (*curr).entity = Box;
        (*box_to).entity = None;
    }

    (*game).score = check_score(cells);
    (*game).playing = true;
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
