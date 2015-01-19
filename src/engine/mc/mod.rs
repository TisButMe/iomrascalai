/************************************************************************
 *                                                                      *
 * Copyright 2014 Urban Hafner                                          *
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

use board::Color;
use board::Move;
use board::Pass;
use game::Game;
use playout::Playout;
use super::Engine;

use std::collections::HashMap;

mod test;

#[derive(Copy)]
struct MoveStats {
    wins: usize,
    plays: usize
}

impl MoveStats {
    pub fn new() -> MoveStats {
        MoveStats { wins: 0, plays: 0 }
    }

    pub fn won(&mut self) {
        self.wins = self.wins + 1;
        self.plays = self.plays + 1;
    }

    pub fn lost(&mut self) {
        self.plays = self.plays + 1;
    }

    pub fn all_wins(&self) -> bool {
        self.wins == self.plays
    }

    pub fn all_loses(&self) -> bool {
        self.wins == 0
    }

    pub fn win_ratio(&self) -> f32 {
        if self.plays == 0 {
            0f32
        } else {
            (self.wins as f32) / (self.plays as f32)
        }
    }
}

pub struct McEngine;

impl McEngine {
    pub fn new() -> McEngine {
        McEngine
    }

}

impl Engine for McEngine {
    fn gen_move(&self, color: Color, game: &Game) -> Move {
        let mut stats = HashMap::new();
        let moves = game.legal_moves();
        for m in moves.iter() {
            stats.insert(m, MoveStats::new());
        }
        for m in moves.iter() {
            let g = game.play(*m).unwrap();
            let playout = Playout::new(g.board());
            for _ in range(0us, 1000) {
                let winner = playout.run();
                let mut prev_move_stats = stats.get_mut(m).unwrap();
                if winner == color {
                    prev_move_stats.won();
                } else {
                    prev_move_stats.lost();
                }
            }
        }
        // pass if 0% wins
        // pass if 100% wins
        if stats.iter().all(|(_, move_stats)| move_stats.all_wins() || move_stats.all_loses()) {
            Pass(color)
        } else {
            let mut m = Pass(color);
            let mut move_stats = MoveStats::new();
            for (m_new, ms) in stats.iter() {
                if ms.win_ratio() > move_stats.win_ratio() {
                    m = **m_new;
                    move_stats = *ms;
                }
            }
            m
        }
    }

}
