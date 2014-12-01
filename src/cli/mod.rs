/************************************************************************
 *                                                                      *
 * Copyright 2014 Urban Hafner, Thomas Poinsot                          *
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
use board::GameAlreadyOver;
use board::IntersectionNotEmpty;
use board::PlayOutOfBoard;
use board::SamePlayerPlayedTwice;
use board::SuicidePlay;
use board::SuperKoRuleBroken;
use board::gomove::Pass;
use board::gomove::Play;
use game::Game;
use ruleset::KgsChinese;
use std::io::stdio::stdin;

pub struct Driver;

impl Driver {
    pub fn new() {
        print!("Please enter the size of the new game: ");
        let mut reader = stdin();

        let size: u8 = match reader.read_line() {
            Ok(l)  => match from_str(l.as_slice().trim_chars('\n')) {
                Some(size) => size,
                None       => fail!("Couldn't convert to a number")
            },
            Err(_) => fail!("Couldn't read the line")
        };

        let mut g = Game::new(size, 6.5, KgsChinese);
        let mut current_player = Black;

        loop {
            if g.is_over() {
                println!("Thanks for playing, score: {}", g.score());
                return;
            }

            print!("{} to play (Enter coordinates separated by space) or p to pass: ", current_player);

            let line = reader.read_line().unwrap();

            let m = if line.as_slice() == "p\n" {
                Pass(current_player)
            } else {
                let coords: Vec<u8> = line.as_slice().trim_chars('\n').split(' ').map(|s| from_str(s).unwrap()).collect();
                Play(current_player, coords[0], coords[1])
            };

            g = match g.play(m) {
                Ok(g)                     => g,
                Err(PlayOutOfBoard)       => fail!("You can't play on invalid coordinates ({} {})", m.coords().col, m.coords().row),
                Err(IntersectionNotEmpty) => fail!("You can't play on a non-empty intersection !"),
                Err(SuicidePlay)          => fail!("You can't play a suicide move with a ruleset forbidding them! ({})", g.ruleset()),
                Err(SamePlayerPlayedTwice)=> fail!("You can't play twice"),
                Err(GameAlreadyOver)      => fail!("You can't play after 2 consecutive passes in TrompTaylor rules"),
                Err(SuperKoRuleBroken)    => fail!("You can't repeat a board position! (Superko rule)")
            };

            current_player = current_player.opposite();

            println!("");
            println!("{}", g);
            g.show_chains();
        }

    }
}
