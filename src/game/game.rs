use crate::{
    ffi::libc::{free, malloc, rand, strcmp},
    game::{
        logic::{
            get_cells, Board, Cell,
            CellBase::{Goal, Wall},
            CellEntity::{Box, None, Player},
            Cells,
        },
        solver::solve,
    },
    structs::{append, pop, Array, Vector2},
    State,
};

#[derive(Clone, Copy, PartialEq)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
    Impossible,
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

    unsafe fn box_count(self, size: i32) -> i32 {
        let requested = match self {
            Difficulty::Easy => 2,
            Difficulty::Medium => 3,
            Difficulty::Hard => 4,
            Difficulty::Impossible => 5,
        };
        let max_inner_cells = (size - 2) * (size - 2);

        clamp(requested, 1, max_inner_cells)
    }

    unsafe fn wall_count(self, size: i32) -> i32 {
        match self {
            Difficulty::Easy => (size + 1) / 2,
            Difficulty::Medium => size,
            Difficulty::Hard => size + (size / 2),
            Difficulty::Impossible => size * 2,
        }
    }

    unsafe fn min_wall_count(self, size: i32) -> i32 {
        match self {
            Difficulty::Easy => size / 3,
            Difficulty::Medium => (size + 1) / 2,
            Difficulty::Hard => size,
            Difficulty::Impossible => size + (size / 2),
        }
    }

    unsafe fn min_pushes(self, size: i32) -> i32 {
        match self {
            Difficulty::Easy => 1,
            Difficulty::Medium => size,
            Difficulty::Hard => size * 2,
            Difficulty::Impossible => size * 3,
        }
    }

    unsafe fn max_pushes(self, size: i32) -> i32 {
        match self {
            Difficulty::Easy => size + 2,
            Difficulty::Medium => (size * 2) + 4,
            Difficulty::Hard => size * 4,
            Difficulty::Impossible => size * 8,
        }
    }

    unsafe fn min_solver_steps(self) -> i32 {
        match self {
            Difficulty::Easy => 0,
            Difficulty::Medium => 500,
            Difficulty::Hard => 2_000,
            Difficulty::Impossible => 5_000,
        }
    }

    unsafe fn max_solver_steps(self) -> i32 {
        match self {
            Difficulty::Easy => 500,
            Difficulty::Medium => 2_000,
            Difficulty::Hard => 5_000,
            Difficulty::Impossible => 20_000,
        }
    }

    unsafe fn max_optimization_attempts(self) -> i32 {
        match self {
            Difficulty::Easy => 8,
            Difficulty::Medium => 16,
            Difficulty::Hard => 24,
            Difficulty::Impossible => 32,
        }
    }

    unsafe fn accepts_solver_steps(self, steps: i32) -> bool {
        steps >= self.min_solver_steps() && steps <= self.max_solver_steps()
    }

    unsafe fn accepts_pushes(self, size: i32, pushes: i32) -> bool {
        pushes >= self.min_pushes(size) && pushes <= self.max_pushes(size)
    }

    unsafe fn accepts_wall_count(self, size: i32, wall_count: usize) -> bool {
        (wall_count as i32) >= self.min_wall_count(size)
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Size {
    Small = 5,
    Medium = 7,
    Large = 9,
    Gigantic = 11,
}

impl Size {
    pub unsafe fn from(input: *const i8) -> Self {
        if strcmp(input, c"small".as_ptr()) == 0 {
            return Size::Small;
        }
        if strcmp(input, c"medium".as_ptr()) == 0 {
            return Size::Medium;
        }
        if strcmp(input, c"large".as_ptr()) == 0 {
            return Size::Large;
        }
        if strcmp(input, c"gigantic".as_ptr()) == 0 {
            return Size::Gigantic;
        }

        Size::Small
    }
}

#[derive(Clone, Copy)]
pub struct Game {
    pub score: i32,
    pub top_score: i32,
    pub top_moves: i32,
    pub playing: bool,
    pub board: *mut Board,
    pub undo_stack: *mut UndoStack,
    pub solution: Array<Vector2>,
    pub solution_index: usize,
    pub solution_tick: i32,
    pub solution_playing: bool,
}

impl Game {
    pub unsafe fn new(size: Size, difficulty: Difficulty, state: *mut State) -> *mut Self {
        let game_ptr = malloc(size_of::<Game>());
        (*state).loading = true;
        let (board, goals, top_moves, solution) = gen_board_and_solve(size, difficulty);
        (*state).loading = false;

        *game_ptr = Self {
            score: 0,
            top_score: goals,
            top_moves,
            playing: true,
            board,
            undo_stack: UndoStack::new(),
            solution,
            solution_index: 0,
            solution_tick: 0,
            solution_playing: false,
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

        if !(*game).undo_stack.is_null() {
            let undo_stack = (*game).undo_stack;
            Array::destroy(&mut (*undo_stack).stack);
            free(undo_stack);
            (*game).undo_stack = core::ptr::null_mut();
        }

        Array::destroy(&mut (*game).solution);

        free(game);
    }
}

unsafe fn gen_board_and_solve(
    size: Size,
    difficulty: Difficulty,
) -> (*mut Board, i32, i32, Array<Vector2>) {
    loop {
        let (player, mut boxes, mut goals, mut walls) = gen_board(size, difficulty);
        let top_score = goals.count as i32;
        let mut optimization_attempts = 0;

        loop {
            let mut solve_result = solve(
                size as i32,
                player,
                boxes,
                goals,
                walls,
                difficulty.max_solver_steps(),
            );

            if solve_result.solved
                && difficulty.accepts_solver_steps(solve_result.explored)
                && difficulty.accepts_pushes(size as i32, solve_result.pushes)
                && difficulty.accepts_wall_count(size as i32, walls.count)
            {
                let board_ptr = malloc(size_of::<Board>());
                *board_ptr = Board::new(size as i32, size as i32, player, boxes, goals, walls);

                Array::destroy(&mut boxes);
                Array::destroy(&mut goals);
                Array::destroy(&mut walls);

                return (board_ptr, top_score, solve_result.moves, solve_result.path);
            }

            if optimization_attempts >= difficulty.max_optimization_attempts() {
                Array::destroy(&mut solve_result.path);
                break;
            }
            optimization_attempts += 1;

            let changed = if solve_result.solved {
                if !difficulty.accepts_wall_count(size as i32, walls.count)
                    || solve_result.pushes < difficulty.min_pushes(size as i32)
                    || solve_result.explored < difficulty.min_solver_steps()
                {
                    increase_level_difficulty(size as i32, player, boxes, goals, &mut walls)
                } else if solve_result.pushes > difficulty.max_pushes(size as i32)
                    || solve_result.explored >= difficulty.max_solver_steps()
                {
                    decrease_level_difficulty(size as i32, difficulty, &mut walls)
                } else {
                    false
                }
            } else if solve_result.explored >= difficulty.max_solver_steps() {
                decrease_level_difficulty(size as i32, difficulty, &mut walls)
            } else {
                false
            };

            Array::destroy(&mut solve_result.path);

            if !changed {
                break;
            }
        }

        Array::destroy(&mut boxes);
        Array::destroy(&mut goals);
        Array::destroy(&mut walls);
    }
}

pub unsafe fn gen_board(
    board_size: Size,
    difficulty: Difficulty,
) -> (Vector2, Array<Vector2>, Array<Vector2>, Array<Vector2>) {
    loop {
        let size = board_size as i32;
        let box_count = difficulty.box_count(size);
        let wall_count = difficulty.wall_count(size);

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

        if boxes_touch_wall_corner(boxes, walls) {
            Array::destroy(&mut boxes);
            Array::destroy(&mut goals);
            Array::destroy(&mut walls);
            continue;
        }

        return (player, boxes, goals, walls);
    }
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

    if total_weight == 0 {
        let index = (rand() as usize) % (*pool).count;
        let picked = *(*pool).items.add(index);
        let last_index = (*pool).count - 1;
        *(*pool).items.add(index) = *(*pool).items.add(last_index);
        (*pool).count -= 1;

        return picked;
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

unsafe fn box_in_wall_corner(walls: Array<Vector2>, box_pos: Vector2) -> bool {
    let left = Vector2::new(box_pos.x - 1, box_pos.y);
    let right = Vector2::new(box_pos.x + 1, box_pos.y);
    let up = Vector2::new(box_pos.x, box_pos.y - 1);
    let down = Vector2::new(box_pos.x, box_pos.y + 1);

    let top_left = has_pos(walls, left) && has_pos(walls, up);
    let top_right = has_pos(walls, up) && has_pos(walls, right);
    let bottom_right = has_pos(walls, right) && has_pos(walls, down);
    let bottom_left = has_pos(walls, down) && has_pos(walls, left);

    top_left || top_right || bottom_right || bottom_left
}

unsafe fn has_pos(arr: Array<Vector2>, pos: Vector2) -> bool {
    for i in 0..arr.count {
        let current = *arr.items.add(i);

        if current.x == pos.x && current.y == pos.y {
            return true;
        }
    }

    false
}

unsafe fn boxes_touch_wall_corner(boxes: Array<Vector2>, walls: Array<Vector2>) -> bool {
    for i in 0..boxes.count {
        let box_pos = *boxes.items.add(i);

        if box_in_wall_corner(walls, box_pos) {
            return true;
        }
    }

    false
}

unsafe fn increase_level_difficulty(
    size: i32,
    player: Vector2,
    boxes: Array<Vector2>,
    goals: Array<Vector2>,
    walls: *mut Array<Vector2>,
) -> bool {
    let mut candidates: Array<Vector2> = Array::new();

    for y in 0..size {
        for x in 0..size {
            let pos = Vector2::new(x, y);

            if pos.x == player.x && pos.y == player.y {
                continue;
            }

            if has_pos(*walls, pos) || has_pos(boxes, pos) || has_pos(goals, pos) {
                continue;
            }

            append(&mut candidates, pos);
        }
    }

    while candidates.count > 0 {
        let wall = pick_weighted(&mut candidates, size, center_weight);
        append(walls, wall);

        if boxes_touch_wall_corner(boxes, *walls) {
            pop(walls);
            continue;
        }

        Array::destroy(&mut candidates);
        return true;
    }

    Array::destroy(&mut candidates);
    false
}

unsafe fn decrease_level_difficulty(
    size: i32,
    difficulty: Difficulty,
    walls: *mut Array<Vector2>,
) -> bool {
    if (*walls).count as i32 <= difficulty.min_wall_count(size) {
        return false;
    }

    let index = (rand() as usize) % (*walls).count;
    let last_index = (*walls).count - 1;
    *(*walls).items.add(index) = *(*walls).items.add(last_index);
    (*walls).count -= 1;

    true
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

pub unsafe fn restart_level(game: *mut Game) {
    (*game).solution_playing = false;
    (*game).solution_index = 0;
    (*game).solution_tick = 0;

    while (*get_undo_stack(game)).stack.count > 0 {
        undo_move(game);
    }

    let cells = *get_cells(get_board(game));
    (*game).score = check_score(cells);
    (*game).playing = true;
}

pub unsafe fn play_solution(game: *mut Game) {
    restart_level(game);

    if (*game).solution.count == 0 {
        return;
    }

    (*game).solution_playing = true;
}

pub unsafe fn step_solution(game: *mut Game) {
    if !(*game).solution_playing {
        return;
    }

    if (*game).solution_index >= (*game).solution.count {
        (*game).solution_playing = false;
        return;
    }

    (*game).solution_tick += 1;
    if (*game).solution_tick < 8 {
        return;
    }
    (*game).solution_tick = 0;

    let delta = *(*game).solution.items.add((*game).solution_index);
    (*game).solution_index += 1;

    match Direction::from_delta(delta) {
        Some(dir) => move_player(game, dir),
        Option::None => {
            (*game).solution_playing = false;
            return;
        }
    }

    if (*game).solution_index >= (*game).solution.count || !(*game).playing {
        (*game).solution_playing = false;
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

    pub unsafe fn from_delta(delta: Vector2) -> Option<Direction> {
        if delta.x == -1 && delta.y == 0 {
            return Some(Direction::Left);
        }
        if delta.x == 1 && delta.y == 0 {
            return Some(Direction::Right);
        }
        if delta.x == 0 && delta.y == -1 {
            return Some(Direction::Up);
        }
        if delta.x == 0 && delta.y == 1 {
            return Some(Direction::Down);
        }

        Option::None
    }
}
