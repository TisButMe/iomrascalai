/************************************************************************
 *                                                                      *
 * Copyright 2014 Urban Hafner                                          *
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
use super::Engine;
use engine::RandomEngine;
use playout::Playout;

use std::collections::hashmap::{HashMap, Vacant, Occupied};

mod test;

struct MoveStats {
    wins: uint,
    plays: uint
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
        self.wins.to_f32().unwrap() / self.plays.to_f32().unwrap()
    }
}

pub struct McEngine {
    random_engine: RandomEngine
}

impl McEngine {
    pub fn new() -> McEngine {
        McEngine{random_engine: RandomEngine::new()}
    }

}

impl Engine for McEngine {
    fn gen_move(&self, color: Color, game: &Game) -> Move {
        let mut stats = HashMap::new();
        let moves = game.legal_moves();
        for m in moves.iter() {
            for i in range(0u, 1) {
                let playout = Playout::new(&self.random_engine);
                let g = game.play(*m).unwrap();
                let winner = playout.run(g);
                let prev_move_stats = match stats.entry(m) {
                    Vacant(entry) => entry.set(MoveStats::new()),
                    Occupied(entry) => entry.into_mut()
                };
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
            let mut mo = Pass(color);
            let mut move_stats = MoveStats::new();
            for (m, ms) in stats.iter() {
                if ms.win_ratio() > move_stats.win_ratio() {
                    mo = **m;
                    move_stats = *ms;
                }
            }
            mo
        }
    }

}
