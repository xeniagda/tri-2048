#![feature(box_syntax)]

#[macro_use]
extern crate lazy_static;
extern crate pancurses;
extern crate rand;

use std::time::Duration;
use std::thread::sleep;
use std::collections::HashMap;

use pancurses::{initscr, endwin, Input, noecho};

mod board;
use board::{Direction, Board, get_random_adds, pick};

lazy_static! {
    static ref KEYMAP: HashMap<Input, Direction> = {
        let mut keymap = HashMap::<Input, Direction>::new();
        keymap.insert(Input::Character('å'), Direction::UpL);
        keymap.insert(Input::Character('ä'), Direction::UpR);
        keymap.insert(Input::Character('a'), Direction::Left);
        keymap.insert(Input::Character('e'), Direction::Right);
        keymap.insert(Input::Character('.'), Direction::DownL);
        keymap.insert(Input::Character('j'), Direction::DownR);
        keymap
    };
}

fn main() {


    let board_choices =
            get_random_adds(Board::empty(4)).into_iter()
            .flat_map(|(prob, board)|
                      board::get_random_adds(board).into_iter().map(move |(prob_, board_)| (prob_ * prob, board_))
                      )
            .collect::<Vec<_>>();

    let mut board = pick(board_choices.as_slice()).clone();

    let win = initscr();

    noecho();

    loop {
        win.clear();
        win.printw(&board.print_board());

        if let Some(ch) = win.getch() {
            if let Some(dir) = KEYMAP.get(&ch) {
                if board.merge(*dir) {
                    board = pick(get_random_adds(board).as_slice()).clone();
                }
            }
        }

        win.refresh();

        sleep(Duration::from_millis(50));
    }

}
