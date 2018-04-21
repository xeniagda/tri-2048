#![feature(box_syntax)]

#[macro_use]
extern crate lazy_static;

use std::sync::{Mutex, Arc};
use std::collections::HashMap;
use std::io::Write;
use std::mem;

#[macro_use]
mod ext;
mod board;

use board::{Direction, Board, get_random_adds, pick};

lazy_static! {
    static ref BOARD: Arc<Mutex<Option<Board>>> = Arc::new(Mutex::new(None));
    static ref KEY_MAP: HashMap<u8, Direction> = {
        let mut keymap = HashMap::new();
        keymap.insert(71, Direction::UpL);   // G
        keymap.insert(72, Direction::Left);  // H
        keymap.insert(77, Direction::DownL); // M
        keymap.insert(82, Direction::UpR);   // L
        keymap.insert(78, Direction::Right); // S
        keymap.insert(86, Direction::DownR); // V
        keymap
    };
}

#[no_mangle]
pub fn start() {
    let board_choices =
            get_random_adds(Board::empty(4)).into_iter()
            .flat_map(|(prob, board)|
                      board::get_random_adds(board).into_iter().map(move |(prob_, board_)| (prob_ * prob, board_))
                      )
            .collect::<Vec<_>>();

    let board = pick(board_choices.as_slice()).clone();

    ext::set_size(board.tiles.len(), board.tiles.last().unwrap().len());

    draw_board(&board);

    let mut board_lock = BOARD.lock().unwrap();
    *board_lock = Some(board);
}

#[no_mangle]
pub fn key_down(key_code: u8) {

    if let Some(dir) = KEY_MAP.get(&key_code) {
        merge(*dir);
    }
}

#[no_mangle]
pub fn merge_dir(dir: u8) {
    let dir = board::DIRECTIONS[dir as usize];

    merge(dir);
}

fn merge(dir: Direction) {
    let mut board_lock = BOARD.lock().unwrap();
    if let Some(ref mut board) = *board_lock {
        if board.merge(dir) {
            let res = pick(&get_random_adds(board.clone())).clone();
            mem::replace(board, res);
        }
        draw_board(&board);
    }
}

fn draw_board(board: &board::Board) {
    ext::clear();
    for y in 0..board.tiles.len() {
        for x in 0..board.tiles[y].len() {
            ext::set(board.tiles[y][x], board.tiles.len() - y - 1, x, board.tiles.len());
        }
    }
}
