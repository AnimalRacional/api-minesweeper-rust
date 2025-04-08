use corsresponder::CORSResponder;
use jzon::object;
use minesweeper::{MinesweeperGame, Squares};
use rocket::{http::Header, tokio::sync::RwLock, Response, State};
use std::{
    collections::HashMap,
    sync::atomic::{AtomicUsize, Ordering},
};

pub mod corsresponder;
pub mod minesweeper;
#[macro_use]
extern crate rocket;

struct Games {
    games: RwLock<HashMap<usize, minesweeper::MinesweeperGame>>,
    next_id: AtomicUsize,
}

impl Games {
    pub fn fetch_next_id(&self) -> usize {
        return self.next_id.fetch_add(1, Ordering::Relaxed);
    }
}

#[get("/new_game/<xsize>/<ysize>/<bomb_amount>")]
async fn new_game(xsize: i32, ysize: i32, bomb_amount: i32, games: &State<Games>) -> CORSResponder {
    if xsize > 0
        && xsize < 100
        && ysize > 0
        && ysize < 100
        && bomb_amount > 0
        && bomb_amount < 100 * 100
    {
        let new_game: MinesweeperGame = MinesweeperGame::new(xsize, ysize, bomb_amount);
        let id: usize = games.fetch_next_id();
        games.games.write().await.insert(id, new_game);
        return CORSResponder::new(id.to_string());
    }
    return CORSResponder::new("failed".to_string());
}

#[get("/action/<id>/<action>/<xpos>/<ypos>")]
async fn action(
    id: usize,
    action: &str,
    xpos: i32,
    ypos: i32,
    games: &State<Games>,
) -> CORSResponder {
    let mut stat = games.games.write().await;
    let game: Option<&mut MinesweeperGame> = stat.get_mut(&id);
    if Option::is_some(&game) {
        let game: &mut MinesweeperGame = game.unwrap();
        if action.starts_with('r') {
            //println!("revealing");
            game.reveal(xpos, ypos);
        } else if action.starts_with('f') {
            //println!("flagging");
            game.flag(xpos, ypos);
        }
        /*
         to return:
         result: success
         state: the board;
         * 0...8 - open, n bombs around
         * 9 - closed
         * 10 - flagged
         * 11 - Bomb that ended the game
        */
        let mut res = vec![
            vec![0u8; game.board_size.x.try_into().unwrap()];
            game.board_size.y.try_into().unwrap()
        ];
        for y in 0..game.board_size.y {
            for x in 0..game.board_size.x {
                let squ = game.get_square(x, y);
                //println!("{} {} {:?}", x,y, squ);
                res[y as usize][x as usize] = match squ {
                    Squares::ClosedBomb => 9,
                    Squares::ClosedSafe => 9,
                    Squares::OpenSafe => game.calculate_neighbours(x, y) as u8,
                    Squares::FlaggedBomb => 10,
                    Squares::FlaggedSafe => 10,
                    Squares::OpenBomb => 11,
                };
            }
        }
        let gamestate = match game.game_state {
            minesweeper::State::Ongoing => "Ongoing",
            minesweeper::State::Won => "Won",
            minesweeper::State::Lost => "Lost",
        };
        if game.game_state != minesweeper::State::Ongoing {
            stat.remove(&id);
        }
        return CORSResponder::new(jzon::stringify(object! {
            result: "success",
            board: res,
            state: gamestate
        }));
    }
    return CORSResponder::new("{\"result\": \"failed\"}".to_string());
}

#[launch]
fn rocket() -> rocket::Rocket<rocket::Build> {
    let games: Games = Games {
        games: RwLock::new(HashMap::new()),
        next_id: AtomicUsize::new(0),
    };
    rocket::build()
        .manage(games)
        .mount("/api", routes![new_game, action])
}
