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

use game::Game;
use playout::Playout;
use ruleset::KgsChinese;

use time::get_time;

pub fn pps(size: u8, runtime: i64) {
    let game = Game::new(size, 6.5, KgsChinese);
    let board = game.board();
    let playout_engine = Playout::new(board);
    let mut counter = 0;
    let start = get_time().sec;

    loop {
        playout_engine.run();
        counter += 1;

        if (get_time().sec - start) >= runtime {
            break;
        }
    }

    println!("Playout per second: {}", counter/runtime);
}
