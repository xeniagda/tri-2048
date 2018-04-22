use std::mem;
use std::iter;
use std::fmt::Write;

use std::io::Write as _Write;

use ext;

#[derive(Debug, Clone)]
pub struct Board {
    pub tiles: Box<[Box<[u8]>]> // Lengths 1, 2, 3, ...
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Right, Left,
    UpR,   UpL,
    DownR, DownL
}

pub const DIRECTIONS: &[Direction] = &[
    Direction::UpL,   Direction::UpR,
    Direction::Left, Direction::Right,
    Direction::DownL, Direction::DownR
];

impl Board {
    pub fn empty(edge_length: u8) -> Board {
        let tiles =
                (1..edge_length+1)
                .map(|len|
                     vec![0; len as usize].into_boxed_slice()
                     )
                .collect::<Vec<_>>();
        Board { tiles: tiles.into_boxed_slice() }
    }

    fn merge_indicies(&mut self, indicies: &[(usize, usize)]) -> bool {
        let values =
                indicies.iter()
                .map(|(y, x)| (&*(&*self.tiles)[*y])[*x]) // ew
                .collect::<Vec<_>>();


        let (merged, moves) = merge(values);

        for (from, to) in moves {
            let from_coords = indicies[from];
            ext::move_tile(self.tiles[from_coords.0][from_coords.1], from_coords, indicies[to]);
        }

        let mut merged_any = false;
        for (value, (y, x)) in merged.into_iter().zip(indicies.iter()) {
            let old = mem::replace(&mut (*(&mut *self.tiles)[*y])[*x], value);
            if old != value {
                merged_any = true
            }

        }
        merged_any
    }

    pub fn merge(&mut self, dir: Direction) -> bool {
        let mut succ = false;
        match dir {
            Direction::Left => {
                for line in 0..self.tiles.len() {
                    let indicies = (0..line+1).into_iter()
                            .map(|x| (line, x))
                            .collect::<Vec<_>>();
                    succ |= self.merge_indicies(indicies.as_slice());
                }
            }
            Direction::Right => {
                for line in 0..self.tiles.len() {
                    let indicies = (0..line+1).into_iter()
                            .map(|x| (line, x))
                            .rev()
                            .collect::<Vec<_>>();
                    succ |= self.merge_indicies(indicies.as_slice());
                }
            }
            Direction::UpL => {
                for line in 0..self.tiles.len() {
                    let indicies = (0..self.tiles.len() - line).into_iter()
                            .map(|x| (line + x, x))
                            .collect::<Vec<_>>();
                    succ |= self.merge_indicies(indicies.as_slice());
                }
            }
            Direction::DownR => {
                for line in 0..self.tiles.len() {
                    let indicies = (0..self.tiles.len() - line).into_iter()
                            .map(|x| (line + x, x))
                            .rev()
                            .collect::<Vec<_>>();
                    succ |= self.merge_indicies(indicies.as_slice());
                }
            }
            Direction::UpR => {
                for line in 0..self.tiles.len() {
                    let indicies = (0..self.tiles.len() - line).into_iter()
                            .map(|x| (x + line, line))
                            .collect::<Vec<_>>();
                    succ |= self.merge_indicies(indicies.as_slice());
                }
            }
            Direction::DownL => {
                for line in 0..self.tiles.len() {
                    let indicies = (0..self.tiles.len() - line).into_iter()
                            .map(|x| (x + line, line))
                            .rev()
                            .collect::<Vec<_>>();
                    succ |= self.merge_indicies(indicies.as_slice());
                }
            }
        }
        succ
    }

    pub fn print_board(&self) -> String {
        let mut res = "".to_string();

        let max_num_width =
                self.tiles.iter()
                .flat_map(|x| x.iter())
                .map(|x| fmt_num(&x).len())
                .max().unwrap();

        for y in 0..self.tiles.len() {
            let leading_space =
                    iter::repeat(' ').take((self.tiles.len() - y - 1) * (max_num_width + 1) / 2).collect::<String>();
            write!(res, "\n{}", leading_space).expect("Can't write!");
            for x in 0..self.tiles[y].len() {
                let num_str = fmt_num(&(*(self.tiles)[y])[x]);
                let space_str = iter::repeat(' ').take(max_num_width - num_str.len()).collect::<String>();
                write!(res, "{}{} ", num_str, space_str).expect("Can't write!");
            }
        }

        res
    }
}

fn fmt_num(n: &u8) -> String {
    if n == &0 { ".".into() }
    else { format!("{}", 1 << (n)) }
}

pub fn merge(mut tiles: Vec<u8>) -> (Vec<u8>, Vec<(usize, usize)>) {

    let orig_tiles = tiles.clone();

    print!("{:?}", tiles);

    let (mut tiles, first_moves) = move_left(tiles);

    print!(" -> {:?}/{:?}", tiles, first_moves);

    let mut merges: Vec<(usize, usize)> = vec![];

    for i in 1..tiles.len() {
        if tiles[i] == tiles[i - 1] && tiles[i] != 0 {
            tiles[i] = 0;
            tiles[i - 1] += 1;

            merges.push((i, i - 1));
        }
    }
    let mut merges_flat =
            (0..tiles.len())
            .map(|idx| merges.iter().find(|(from, to)| *from == idx).map(|x| x.1).unwrap_or(idx))
            .collect::<Vec<_>>();

    print!(" -> {:?}/{:?}", tiles, merges_flat);

    let (tiles, second_moves) = move_left(tiles);

    print!(" -> {:?}/{:?}", tiles, second_moves);

    let all_merges =
            (0..tiles.len())
            .map(|x| (x, second_moves[merges_flat[first_moves[x]]]))
            .filter(|(from, to)| from != to && orig_tiles[*from] != 0)
            .collect();

    println!(" -> {:?}", all_merges);

    (tiles, all_merges)
}

pub fn move_left(mut tiles: Vec<u8>) -> (Vec<u8>, Vec<usize>) {
    let mut move_lefts: Vec<usize> = vec![];

    for i in 0..tiles.len() {
        let mut n = i;
        while n > 0 {
            if tiles[n - 1] != 0 {
                break;
            }
            tiles.swap(n, n - 1);
            n -= 1;
        }
        move_lefts.push(n);
    }

    (tiles, move_lefts)
}

pub fn get_random_adds(board: Board) -> Vec<(f32, (Board, (usize, usize)))> {
    let mut total_placeable = 0;
    for y in 0..board.tiles.len() {
        for x in 0..y+1 {
            if board.tiles[y][x] == 0 {
                total_placeable += 1;
            }
        }
    }

    let mut res = vec![];
    for y in 0..board.tiles.len() {
        for x in 0..y+1 {
            if board.tiles[y][x] == 0 {
                let mut new_board = board.clone();
                (*(&mut *new_board.tiles)[y])[x] = 1;
                res.push((0.9 / total_placeable as f32, (new_board, (y, x))));

                let mut new_board = board.clone();
                (*(&mut *new_board.tiles)[y])[x] = 2;
                res.push((0.1 / total_placeable as f32, (new_board, (y, x))));
            }
        }
    }
    res
}

pub fn pick<'a, T>(lst: &'a [(f32, T)]) -> &'a T {
    let mut curr_prob = 1.;
    for (prob, item) in lst.iter() {
        let num = ext::rand();
        if &(num * curr_prob) < prob {
            return item;
        }
        curr_prob = curr_prob - prob;
    }
    writeln!(ext::JSLog, "o no");
    unreachable!()
}

#[test]
fn test_fmt_num() {
    assert_eq!(".".to_string(), fmt_num(&0));
    assert_eq!("2".to_string(), fmt_num(&1));
    assert_eq!("64".to_string(), fmt_num(&6));
    assert_eq!("8".to_string(), fmt_num(&3));
}

#[test]
fn test_board_empty() {
    let board_5 = Board::empty(5);
    assert_eq!(5, board_5.tiles.len());
    for x in 0..board_5.tiles.len() {
        assert_eq!(x + 1, board_5.tiles[x].len());
    }
}

#[test]
fn test_board_merged_indicies() {
    let board =
        Board {
            tiles:
            box [
                box [ 5 ],
                box [ 1, 1 ],
                box [ 1, 1, 0 ],
                box [ 2, 1, 1, 0 ]
            ]
        };

    { // Left
        let mut board = board.clone();

        assert!(board.merge(Direction::Left));

        // Need to assign to variable as rustc can't figure out the type of &[&[x], &[y, z]]
        let wanted: Box<[Box<[u8]>]> =
            box [
                box [ 5 ],
                box [ 2, 0 ],
                box [ 2, 0, 0 ],
                box [ 2, 2, 0, 0 ]
            ];

        assert_eq!(wanted,
                   board.tiles
                  );
    }
    { // Right
        let mut board = board.clone();

        assert!(board.merge(Direction::Right));

        let wanted: Box<[Box<[u8]>]> =
            box [
                box [ 5 ],
                box [ 0, 2 ],
                box [ 0, 0, 2 ],
                box [ 0, 0, 2, 2 ]
            ];

        assert_eq!(wanted,
                   board.tiles
                  );
    }
    { // UpL
        let mut board = board.clone();

        assert!(board.merge(Direction::UpL));

        let wanted: Box<[Box<[u8]>]> =
            box [
                box [ 5 ],
                box [ 2, 1 ],
                box [ 2, 1, 0 ],
                box [ 2, 0, 0, 0 ]
            ];

        assert_eq!(wanted,
                   board.tiles
                  );
    }
    { // DownR
        let mut board = board.clone();

        assert!(board.merge(Direction::DownR));

        let wanted: Box<[Box<[u8]>]> =
            box [
                box [ 0 ],
                box [ 0, 0 ],
                box [ 0, 1, 5 ],
                box [ 2, 2, 2, 1 ]
            ];

        assert_eq!(wanted,
                   board.tiles
                  );
    }
    { // UpR
        let mut board = board.clone();

        assert!(board.merge(Direction::UpR));

        let wanted: Box<[Box<[u8]>]> =
            box [
                box [ 5 ],
                box [ 2, 2 ],
                box [ 2, 1, 1 ],
                box [ 0, 0, 0, 0 ]
            ];

        assert_eq!(wanted,
                   board.tiles
                  );
    }
    { // Down
        let mut board = board.clone();

        assert!(board.merge(Direction::DownL));

        let wanted: Box<[Box<[u8]>]> =
            box [
                box [ 0 ],
                box [ 5, 0 ],
                box [ 2, 1, 0 ],
                box [ 2, 2, 1, 0 ]
            ];

        assert_eq!(wanted,
                   board.tiles
                  );
    }
}

#[test]
fn test_merge() {
    assert_eq!((vec![2, 0, 0, 0], vec![(1, 0)]), merge(vec![1, 1, 0, 0]));
    assert_eq!((vec![2, 2, 0, 0], vec![(1, 0), (2, 1), (3, 1)]), merge(vec![1, 1, 1, 1]));
    assert_eq!((vec![2, 2, 0, 0], vec![(1, 0), (2, 0), (3, 1)]), merge(vec![0, 1, 1, 2]));

    assert_eq!((vec![2, 0, 0, 0], vec![(1, 0), (2, 0)]), merge(vec![0, 1, 1, 0]));
    assert_eq!((vec![2, 2, 0, 0], vec![(2, 1), (3, 1)]), merge(vec![2, 0, 1, 1]));

    assert_eq!((vec![4, 3, 2, 0], vec![(1, 0), (2, 1), (3, 2)]), merge(vec![3, 3, 3, 2]));
    assert_eq!((vec![1, 5, 0, 0], vec![(2, 0), (3, 1)]), merge(vec![0, 0, 1, 5]));
    assert_eq!((vec![2, 0, 0, 0], vec![(2, 0)]), merge(vec![1, 0, 1, 0]));
}
