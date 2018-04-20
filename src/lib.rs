#![feature(box_syntax)]

#[macro_use]
extern crate lazy_static;

use std::io::Write;

mod ext;
mod board;

use board::{Direction, Board, get_random_adds, pick};

#[no_mangle]
pub fn start() {
    writeln!(ext::JSLog, "Starting");

    writeln!(ext::JSLog, "Random float: {}", ext::rand());

    writeln!(ext::JSLog, "Done");

    let board_choices =
            get_random_adds(Board::empty(4)).into_iter()
            .flat_map(|(prob, board)|
                      board::get_random_adds(board).into_iter().map(move |(prob_, board_)| (prob_ * prob, board_))
                      )
            .collect::<Vec<_>>();

    let mut board = pick(board_choices.as_slice()).clone();

    draw_board(&board);
}

fn draw_board(board: &board::Board) {
    for y in 0..board.tiles.len() {
        for x in 0..board.tiles[y].len() + 1 {
            ext::set(board.tiles[y][x], y, x);
        }
    }
}
