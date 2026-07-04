use crate::structs::{append, Array, Vector2};

#[derive(Clone, Copy)]
struct SolverState {
    player: Vector2,
    boxes: Array<Vector2>,
    depth: i32,
    pushes: i32,
    path: Array<Vector2>,
}

#[derive(Clone, Copy)]
pub struct SolveResult {
    pub solved: bool,
    pub moves: i32,
    pub pushes: i32,
    pub explored: i32,
    pub path: Array<Vector2>,
}

pub unsafe fn solve(
    size: i32,
    player: Vector2,
    boxes: Array<Vector2>,
    goals: Array<Vector2>,
    walls: Array<Vector2>,
    max_steps: i32,
) -> SolveResult {
    let start = SolverState {
        player,
        boxes: clone_boxes(boxes),
        depth: 0,
        pushes: 0,
        path: Array::new(),
    };

    let mut queue: Array<SolverState> = Array::new();
    let mut visited: Array<SolverState> = Array::new();

    append(&mut queue, start);
    append(&mut visited, start);

    let mut cursor: usize = 0;
    let mut steps: i32 = 0;

    while cursor < queue.count && steps < max_steps {
        let state = *queue.items.add(cursor);
        cursor += 1;
        steps += 1;

        if all_boxes_on_goals(state.boxes, goals) {
            return SolveResult {
                solved: true,
                moves: state.depth,
                pushes: state.pushes,
                explored: steps,
                path: state.path,
            };
        }

        for dir in 0..4 {
            let delta = match dir {
                0 => Vector2::new(-1, 0),
                1 => Vector2::new(1, 0),
                2 => Vector2::new(0, -1),
                _ => Vector2::new(0, 1),
            };

            let target = Vector2::new(state.player.x + delta.x, state.player.y + delta.y);

            if !in_board(target, size) || is_wall(walls, target) {
                continue;
            }

            let mut next_state = SolverState {
                player: target,
                boxes: clone_boxes(state.boxes),
                depth: state.depth + 1,
                pushes: state.pushes,
                path: clone_path(state.path),
            };
            append(&mut next_state.path, delta);

            if is_box(state.boxes, target) {
                let box_target = Vector2::new(target.x + delta.x, target.y + delta.y);

                if !in_board(box_target, size)
                    || is_wall(walls, box_target)
                    || is_box(state.boxes, box_target)
                {
                    Array::destroy(&mut next_state.boxes);
                    Array::destroy(&mut next_state.path);
                    continue;
                }

                move_box(next_state.boxes, target, box_target);
                next_state.pushes += 1;
            }

            sort_boxes(next_state.boxes);

            if state_seen(visited, next_state) {
                Array::destroy(&mut next_state.boxes);
                Array::destroy(&mut next_state.path);
                continue;
            }

            append(&mut queue, next_state);
            append(&mut visited, next_state);
        }
    }

    SolveResult {
        solved: false,
        moves: -1,
        pushes: -1,
        explored: steps,
        path: Array::new(),
    }
}

unsafe fn is_wall(walls: Array<Vector2>, pos: Vector2) -> bool {
    has_pos(walls, pos)
}

unsafe fn is_box(boxes: Array<Vector2>, pos: Vector2) -> bool {
    has_pos(boxes, pos)
}

unsafe fn is_goal(goals: Array<Vector2>, pos: Vector2) -> bool {
    has_pos(goals, pos)
}

unsafe fn in_board(pos: Vector2, size: i32) -> bool {
    pos.x >= 0 && pos.y >= 0 && pos.x < size && pos.y < size
}

unsafe fn state_equals(a: SolverState, b: SolverState) -> bool {
    if !vec_equals(a.player, b.player) || a.boxes.count != b.boxes.count {
        return false;
    }

    for i in 0..a.boxes.count {
        let a_box = *a.boxes.items.add(i);
        let b_box = *b.boxes.items.add(i);

        if !vec_equals(a_box, b_box) {
            return false;
        }
    }

    true
}

unsafe fn state_seen(visited: Array<SolverState>, state: SolverState) -> bool {
    for i in 0..visited.count {
        let current = *visited.items.add(i);
        if state_equals(current, state) {
            return true;
        }
    }

    false
}

unsafe fn clone_boxes(boxes: Array<Vector2>) -> Array<Vector2> {
    let mut cloned = Array::new();

    for i in 0..boxes.count {
        append(&mut cloned, *boxes.items.add(i));
    }

    sort_boxes(cloned);
    cloned
}

unsafe fn clone_path(path: Array<Vector2>) -> Array<Vector2> {
    let mut cloned = Array::new();

    for i in 0..path.count {
        append(&mut cloned, *path.items.add(i));
    }

    cloned
}

unsafe fn move_box(boxes: Array<Vector2>, from: Vector2, to: Vector2) {
    for i in 0..boxes.count {
        let current = boxes.items.add(i);

        if vec_equals(*current, from) {
            *current = to;
            return;
        }
    }
}

unsafe fn all_boxes_on_goals(boxes: Array<Vector2>, goals: Array<Vector2>) -> bool {
    for i in 0..boxes.count {
        let box_pos = *boxes.items.add(i);

        if !is_goal(goals, box_pos) {
            return false;
        }
    }

    true
}

unsafe fn has_pos(arr: Array<Vector2>, pos: Vector2) -> bool {
    for i in 0..arr.count {
        let current = *arr.items.add(i);

        if vec_equals(current, pos) {
            return true;
        }
    }

    false
}

unsafe fn vec_equals(a: Vector2, b: Vector2) -> bool {
    a.x == b.x && a.y == b.y
}

unsafe fn sort_boxes(boxes: Array<Vector2>) {
    if boxes.count < 2 {
        return;
    }

    for i in 0..boxes.count {
        for j in (i + 1)..boxes.count {
            let a = *boxes.items.add(i);
            let b = *boxes.items.add(j);

            if b.y < a.y || (b.y == a.y && b.x < a.x) {
                *boxes.items.add(i) = b;
                *boxes.items.add(j) = a;
            }
        }
    }
}
