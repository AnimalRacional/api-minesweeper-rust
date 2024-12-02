use rand::{thread_rng, Rng};

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Squares{
    ClosedSafe,
    OpenSafe,
    FlaggedSafe,
    ClosedBomb,
    FlaggedBomb,
    OpenBomb
}
#[derive(Eq, PartialEq)]
pub enum State{
    Ongoing,
    Won,
    Lost
}

pub struct MinesweeperGame{
    pub board_size: Point,
    pub board: Vec<Squares>,
    pub ori_bomb_amount: i32,
    pub game_state: State
}

pub struct Point {
    pub x: i32,
    pub y: i32
}

impl Point{
    pub fn new(x: i32, y: i32) -> Point{
        Point{ x: x, y: y }
    }
}

// Setting up
impl MinesweeperGame {
    pub fn new(xsize: i32, ysize: i32, bomb_amount: i32) -> MinesweeperGame{
        let mut game = MinesweeperGame {
            board_size: Point::new(xsize, ysize),
            board: vec![Squares::ClosedSafe],
            ori_bomb_amount: if bomb_amount < xsize * ysize { bomb_amount } else { xsize * ysize - 1 },
            game_state: State::Ongoing
        };
        game.reset_board();
        game
    }

    fn reset_board(&mut self){
        self.board.clear();
        let board_size: i32 = self.board_size.x * self.board_size.y;
        for _ in 0..board_size {
            self.board.push(Squares::ClosedSafe);
        }

        for _ in 0..self.ori_bomb_amount {
            let mut id = thread_rng().gen_range(0..self.board.len());
            while let Squares::ClosedBomb = self.board[id] {
                id = thread_rng().gen_range(0..self.board.len());
            }
            self.board[id] = Squares::ClosedBomb;
        }
    }
}

// Util
impl MinesweeperGame {
    fn pos_moves() -> Vec<Point> {
        vec![
            Point::new(-1, -1),
            Point::new(-1, 0),
            Point::new(-1, 1),
            Point::new(0, -1),
            Point::new(0, 1),
            Point::new(1, -1),
            Point::new(1, 0),
            Point::new(1, 1)
        ]
    }

    pub fn calculate_index_by_coords(&self, x: i32, y: i32) -> i32{
        y * self.board_size.x + x
    }

    pub fn calculate_coords_by_index(&self, i: i32) -> Point{
        Point::new(i % self.board_size.x, i / self.board_size.x)
    }

    pub fn is_inside(&self, x: i32, y: i32) -> bool{
        x < self.board_size.x && x >= 0 && y < self.board_size.y && y >= 0
    }
    
    pub fn calculate_generic_neighbours(&self, x: i32, y: i32, flagged_safe: bool, 
                                    flagged_bomb: bool, closed_safe: bool, closed_bomb: bool, 
                                    open_safe: bool, open_bomb: bool) -> i32 {
        let mut res = 0;
        let pos_moves = MinesweeperGame::pos_moves();
        for i in pos_moves {
            if self.is_inside(x + i.x, y + i.y) {
                let id = self.calculate_index_by_coords(x + i.x, y + i.y);
                if let Some(a) = self.board.get(id as usize) {
                    let mut found = false;
                    if flagged_safe && (Squares::FlaggedSafe == *a){
                        found = true;
                    }
                    else if flagged_bomb && (Squares::FlaggedBomb == *a){
                        found = true;
                    }
                    else if closed_safe && (Squares::ClosedSafe == *a){
                        found = true;
                    }
                    else if closed_bomb && (Squares::ClosedBomb == *a){
                        found = true;
                    }
                    else if open_safe && (Squares::OpenSafe == *a){
                        found = true;
                    }
                    else if open_bomb && (Squares::OpenBomb == *a){
                        found = true;
                    }
                    if found {
                        res += 1;
                    }
                }
            }
        }
        res
    }
    pub fn calculate_neighbours(&self, x: i32, y: i32) -> i32 {        
        self.calculate_generic_neighbours(x, y, false, true, false, true, false, true)
    }
    pub fn calculate_flag_neighbours(&self, x: i32, y: i32) -> i32{
        self.calculate_generic_neighbours(x, y, true, true, false, false, false, false)
    }
    pub fn get_square(&self, x: i32, y: i32) -> Squares {
        self.board[self.calculate_index_by_coords(x, y) as usize]
    }
}

// Game
impl MinesweeperGame {
    pub fn restart_game(&mut self){
        self.game_state = State::Ongoing;
        self.reset_board();
    }

    pub fn lose(&mut self){
        self.game_state = State::Lost;
    }

    pub fn win(&mut self){
        self.game_state = State::Won;
    }
    
    fn reveal_mayclick(&mut self, xpos: i32, ypos: i32, click: bool){
        let id: usize = self.calculate_index_by_coords(xpos, ypos) as usize;
        self.board[id] = match &self.board[id] {
            Squares::ClosedBomb => { self.lose(); Squares::OpenBomb }
            Squares::ClosedSafe => { 
                self.reveal_around(xpos, ypos);
                Squares::OpenSafe 
            }
            // If clicking on a number
            Squares::OpenSafe => {
                if click{
                    let bombs_around: i32 = self.calculate_neighbours(xpos, ypos);
                    let flags_around: i32 = self.calculate_flag_neighbours(xpos, ypos);
                    if bombs_around == flags_around {
                        self.reveal_around_ignore(xpos, ypos, true);
                    }
                }
                Squares::OpenSafe
            }
            other => { *other }
        };
        if self.has_won() {
            self.win();
        }
    }

    pub fn reveal(&mut self, xpos: i32, ypos: i32){
        if self.game_state == State::Ongoing {
            self.reveal_mayclick(xpos, ypos, true);
        }
    }

    fn reveal_around(&mut self, x: i32, y: i32){
        self.reveal_around_ignore(x, y, false);
    }   

    fn reveal_around_ignore(&mut self, x: i32, y: i32, ignore_bombs: bool){
        let pos_moves = MinesweeperGame::pos_moves();
        if self.is_inside(x, y){
            if if ignore_bombs { true } else { self.calculate_neighbours(x, y) <= 0 } {
                for i in pos_moves {
                    if self.is_inside(x + i.x, y + i.y) {
                        let pos = self.calculate_index_by_coords(x + i.x, y + i.y);
                        if let Some(a) = self.board.get(pos as usize){
                            if let Squares::ClosedSafe = a {
                                self.board[pos as usize] = Squares::OpenSafe;
                                //println!("Started revealing at ({}, {}) with {} neighbours origin ({}, {})", x + i.x, y + i.y, self.calculate_neighbours(x, y), x, y);
                                self.reveal_around(x + i.x, y + i.y);
                            }
                            else if let Squares::ClosedBomb = a {
                                self.board[pos as usize] = Squares::OpenBomb;
                                self.lose();
                                return;
                            }
                        }
                    }
                }
            }
            let pos = self.calculate_index_by_coords(x, y);
            self.board[pos as usize] = Squares::OpenSafe;
        }
        if self.has_won() {
            self.win();
        }
    }

    pub fn flag(&mut self, xpos: i32, ypos: i32){
        if self.game_state == State::Ongoing{
            let id = self.calculate_index_by_coords(xpos, ypos) as usize;
            self.board[id] = match &self.board[id]{
                Squares::ClosedBomb => { Squares::FlaggedBomb },
                Squares::ClosedSafe => { Squares::FlaggedSafe },
                Squares::FlaggedBomb => { Squares::ClosedBomb },
                Squares::FlaggedSafe => { Squares::ClosedSafe },
                other => { *other }
            };
            if self.has_won() {
                self.win();
            }
        }
    }

    pub fn has_won(&self) -> bool {
        let mut found_bomb = false;
        let mut found_nonrevealed = false;
        for i in &self.board{
            if *i == Squares::ClosedBomb {
                found_bomb = true;
            }
            else if *i == Squares::FlaggedSafe || *i == Squares::ClosedSafe {
                found_nonrevealed = true;
            }
            if found_bomb && found_nonrevealed {
                return false;
            }
        }
        true
    }
}