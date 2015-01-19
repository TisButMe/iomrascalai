/************************************************************************
 *                                                                      *
 * Copyright 2014 Urban Hafner, Thomas Poinsot                          *
 * Copyright 2015 Urban Hafner, Thomas Poinsot                          *
 *                                                                      *
 * This file is part of Iomrascálaí.                                    *
 *                                                                      *
 * Iomrascálaí is free software: you can redistribute it and/or modify  *
 * it under the terms of the GNU General Public License as published by *
 * the Free Software Foundation, either version 3 of the License, or    *
 * (at your option) any later version.                                  *
 *                                                                      *
 * Iomrascálaí is distributed in the hope that it will be useful,       *
 * but WITHOUT ANY WARRANTY; without even the implied warranty of       *
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the        *
 * GNU General Public License for more details.                         *
 *                                                                      *
 * You should have received a copy of the GNU General Public License    *
 * along with Iomrascálaí.  If not, see <http://www.gnu.org/licenses/>. *
 *                                                                      *
 ************************************************************************/

use board::Black;
use board::Board;
use board::Color;
use board::Coord;
use board::Empty;
use board::IllegalMove;
use board::Move;
use board::Play;
use board::White;
use ruleset::Ruleset;
use score::Score;
use self::hash::ZobristHashTable;

use std::fmt;
use core::fmt::String;
use std::rc::Rc;

mod hash;
mod test;

#[derive(Clone)]
pub struct Game<'a> {
    board: Board<'a>,
    move_number: u8,
    previous_boards_hashes: Vec<u64>,
    zobrist_base_table: Rc<ZobristHashTable>,
    main_time: u64, // main time in ms
    byo_time: u64, // byo yomi time in ms
    byo_stones: i32, // stones per byo yomi period
    byo_stones_remaining: i32
}

impl<'a> Game<'a> {
    pub fn new<'b>(size: u8, komi: f32, ruleset: Ruleset) -> Game<'b> {
        let zobrist_base_table = Rc::new(ZobristHashTable::new(size));
        let new_board = Board::new(size, komi, ruleset);

        Game {
            board: new_board,
            move_number: 0,
            previous_boards_hashes: vec!(zobrist_base_table.init_hash()),
            zobrist_base_table: zobrist_base_table,
            main_time: 0,
            byo_time: 30,
            byo_stones: 1,
            byo_stones_remaining: 1
        }
    }

    pub fn play<'b>(&'b self, m: Move) -> Result<Game<'b>, IllegalMove> {
        let mut new_board = self.board.clone();

        match new_board.play(m) {
            Ok(_) => {
                let mut new_game_state = self.clone();
                new_game_state.board = new_board;
                new_game_state.move_number += 1;
                new_game_state.byo_stones_remaining = if self.byo_stones_remaining == 1 {
                    self.byo_stones
                } else {
                    self.byo_stones_remaining - 1
                };
                
                if !m.is_pass() {
                    let hash = new_game_state.compute_hash(&m);
                    if new_game_state.previous_boards_hashes.contains(&hash) {
                        return Err(IllegalMove::SuperKo)
                    }
                    new_game_state.previous_boards_hashes.push(hash);
                }
                Ok(new_game_state)
            },
            Err(m) => Err(m)
        }
    }

    fn compute_hash(&self, m: &Move) -> u64 {
        let mut hash = self.zobrist_base_table.add_stone_to_hash(*self.previous_boards_hashes.last().unwrap(), m);
        for &coord in self.board.adv_stones_removed().iter() {
            hash = self.zobrist_base_table.remove_stone_from_hash(hash, &Play(m.color().opposite(), coord.col, coord.row));
        }
        for &coord in self.board.friend_stones_removed().iter() {
            hash = self.zobrist_base_table.remove_stone_from_hash(hash, &Play(*m.color(), coord.col, coord.row));
        }
        hash
    }

    // Note: This method uses 1-1 as the origin point, not 0-0. 19-19 is a valid coordinate in a 19-sized board, while 0-0 is not.
    //       this is done because I think it makes more sense in the context of go. (Least surprise principle, etc...)
    pub fn get(&self, col: u8, row: u8) -> Color {
        self.board.color(&Coord::new(col, row))
    }

    pub fn ruleset(&self) -> Ruleset {
        self.board.ruleset()
    }

    pub fn move_number(&self) -> u8 {
        self.move_number
    }

    pub fn is_over(&self) -> bool {
        self.board.is_game_over()
    }

    pub fn komi(&self) -> f32 {
        self.board.komi()
    }

    pub fn main_time(&self) -> u64 {
        self.main_time
    }

    pub fn byo_time(&self) -> u64 {
        self.byo_time
    }

    pub fn byo_stones(&self) -> i32 {
        self.byo_stones
    }

    pub fn size(&self) -> u8 {
        self.board.size()
    }

    pub fn score(&self) -> Score {
        self.board.score()
    }

    pub fn winner(&self) -> Color {
        self.board.winner()
    }

    pub fn set_komi(&mut self, komi: f32) {
        self.board.set_komi(komi);
    }

    pub fn set_main_time(&mut self, time: u64) {
        self.main_time = time
    }

    pub fn set_byo_time(&mut self, time: u64) {
        self.byo_time = time;
    }

    pub fn set_byo_stones(&mut self, stones: i32) {
        let actual = if stones < 1 { 1 } else { stones };
        self.byo_stones = actual;
        self.byo_stones_remaining = actual;
    }

    pub fn board_size(&self) -> u8 {
        self.board.size()
    }

    pub fn board(&self) -> Board {
        self.board.clone()
    }

    pub fn show_chains(&self) {
        for c in self.board.chains().iter() {
            println!("{}", c.show());
        }
    }

    pub fn next_player(&self) -> Color {
        self.board.next_player()
    }

    pub fn legal_moves(&self) -> Vec<Move> {
        self.board.legal_moves()
    }
}

impl<'a> String for Game<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = format!("komi: {}\n", self.komi());

        s.push_str(self.board.as_string().as_slice());

        // Then we print the col numbers under the board
        s.push_str(format!("{:3}", "").as_slice());
        for col in range(1, self.board.size()+1) {
            s.push_str(format!("{:<2}", col).as_slice());
        }
        s.push_str("\n");
        s.fmt(f)
    }
}
