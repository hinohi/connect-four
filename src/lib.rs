const COLUMNS: usize = 7;
const ROWS: usize = 6;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Side {
    Side1,
    Side2,
}

impl Side {
    pub fn flip(self) -> Side {
        match self {
            Side::Side1 => Side::Side2,
            Side::Side2 => Side::Side1,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Cell {
    Empty,
    Put(Side),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum BattleState {
    Doing,
    Winner(Side),
    Draw,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Board {
    side: Side,
    columns: [[Cell; ROWS]; COLUMNS],
}

impl Board {
    pub fn new() -> Board {
        Board {
            side: Side::Side1,
            columns: [[Cell::Empty; ROWS]; COLUMNS],
        }
    }

    pub fn list_next(&self) -> Vec<Board> {
        let mut ret = Vec::with_capacity(8);
        'OUTER: for c in 0..COLUMNS {
            for r in 0..ROWS {
                if self.columns[c][r] == Cell::Empty {
                    let mut board = self.clone();
                    board.columns[c][r] = Cell::Put(self.side);
                    board.side = board.side.flip();
                    ret.push(board);
                    continue 'OUTER;
                }
            }
        }
        ret
    }

    pub fn state(&self) -> BattleState {
        static SIDE1: &[Cell] = &[Cell::Put(Side::Side1); 4];
        static SIDE2: &[Cell] = &[Cell::Put(Side::Side2); 4];
        for column in self.columns.iter() {
            for i in 0..ROWS - 4 {
                if &column[i..i + 4] == SIDE1 {
                    return BattleState::Winner(Side::Side1);
                }
                if &column[i..i + 4] == SIDE2 {
                    return BattleState::Winner(Side::Side2);
                }
            }
        }

        for c in 0..COLUMNS - 4 {
            let columns = &self.columns[c..c + 4];
            for r in 0..ROWS {
                if columns.iter().all(|col| col[r] == Cell::Put(Side::Side1)) {
                    return BattleState::Winner(Side::Side1);
                }
                if columns.iter().all(|col| col[r] == Cell::Put(Side::Side2)) {
                    return BattleState::Winner(Side::Side2);
                }
            }
            for r in 0..ROWS - 4 {
                if columns
                    .iter()
                    .enumerate()
                    .all(|(i, col)| col[r + i] == Cell::Put(Side::Side1))
                {
                    return BattleState::Winner(Side::Side1);
                }
                if columns
                    .iter()
                    .enumerate()
                    .all(|(i, col)| col[r + i] == Cell::Put(Side::Side2))
                {
                    return BattleState::Winner(Side::Side2);
                }
            }
            for r in 0..ROWS - 4 {
                if columns
                    .iter()
                    .enumerate()
                    .all(|(i, col)| col[r + 3 - i] == Cell::Put(Side::Side1))
                {
                    return BattleState::Winner(Side::Side1);
                }
                if columns
                    .iter()
                    .enumerate()
                    .all(|(i, col)| col[r + 3 - i] == Cell::Put(Side::Side2))
                {
                    return BattleState::Winner(Side::Side2);
                }
            }
        }
        if self.columns.iter().all(|col| col[ROWS - 1] != Cell::Empty) {
            BattleState::Draw
        } else {
            BattleState::Doing
        }
    }
}

pub fn full_search(board: &Board) -> i8 {
    ab_search(board, 0, -100, 100)
}

fn ab_search(board: &Board, depth: i8, alpha: i8, beta: i8) -> i8 {
    match board.state() {
        BattleState::Winner(_) => return depth - 100,
        BattleState::Draw => return 0,
        _ => (),
    }
    let mut alpha = alpha;
    for next in board.list_next() {
        let s = -ab_search(&next, depth + 1, -beta, -alpha);
        if s > alpha {
            alpha = s;
        }
        if alpha >= beta {
            break;
        }
    }
    if depth < 16 {
        println!(
            "{}{} {}",
            String::from_utf8(vec![32; (depth * 2) as usize]).unwrap(),
            alpha,
            beta,
        );
    }
    alpha
}
