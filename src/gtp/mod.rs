/************************************************************************
 *                                                                      *
 * Copyright 2014 Thomas Poinsot, Urban Hafner                          *
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
use board::gomove::Move;
use engine::Engine;
use game::Game;
use ruleset::KgsChinese;

pub mod driver;
mod test;

#[deriving(Show)]
pub enum Command {
    Play,
    PlayError(Move),
    GenMove(String),
    GenMoveError(Move),
    ProtocolVersion,
    Name,
    Version,
    KnownCommand(bool),
    ListCommands(String),
    Quit,
    BoardSize,
    ClearBoard,
    Komi,
    ShowBoard(String),
    Empty,
    Error,
    FinalScore(String)
}

pub struct GTPInterpreter<'a> {
    known_commands: Vec<String>,
    game: Game<'a>,
    engine: Box<Engine>
}

impl<'a> GTPInterpreter<'a > {
    pub fn new<'a>(engine: Box<Engine>) -> GTPInterpreter<'a> {
        let komi = 6.5;
        let boardsize = 19;
        let mut interpreter = GTPInterpreter {
            known_commands: vec!(),
            game: Game::new(boardsize, komi, KgsChinese),
            engine: engine
        };
        interpreter.initialize();
        interpreter
    }

    fn initialize(&mut self) {
        self.known_commands = self.generate_known_commands();
    }

    fn generate_known_commands(&self) -> Vec<String> {
        let mut known_commands = Vec::new();
        known_commands.push(String::from_str("play"));
        known_commands.push(String::from_str("genmove"));
        known_commands.push(String::from_str("protocol_version"));
        known_commands.push(String::from_str("name"));
        known_commands.push(String::from_str("version"));
        known_commands.push(String::from_str("known_command"));
        known_commands.push(String::from_str("list_commands"));
        known_commands.push(String::from_str("quit"));
        known_commands.push(String::from_str("boardsize"));
        known_commands.push(String::from_str("clear_board"));
        known_commands.push(String::from_str("komi"));
        known_commands.push(String::from_str("showboard"));
        known_commands.push(String::from_str("final_score"));
        known_commands
    }

    pub fn game<'b>(&'b self) -> &'b Game {
        &self.game
    }

    pub fn komi(&self) -> f32 {
        self.game.komi()
    }

    pub fn boardsize(&self) -> u8 {
        self.game.size()
    }

    pub fn read(&mut self, input: &str) -> Command {
        let preprocessed = self.preprocess(input);

        if preprocessed.len() == 0 {return Empty};

        let command: Vec<&str> = preprocessed.as_slice().split(' ').collect();

        match command[0] {
            "name"             => return Name,
            "version"          => return Version,
            "protocol_version" => return ProtocolVersion,
            "list_commands"    => return ListCommands(self.list_commands()),
            "known_command"    => return KnownCommand(self.known_commands.contains(&String::from_str(command[1].clone()))),
            "boardsize"        => return match from_str::<u8>(command[1]) {
                Some(size) => {
                    self.game = Game::new(size, self.komi(), KgsChinese);
                    BoardSize
                },
                None       => Error
            },
            "clear_board"      => {
                self.game = Game::new(self.boardsize(), self.komi(), KgsChinese);
                ClearBoard
            },
            "komi"             => return match from_str::<f32>(command[1]) {
                Some(komi) => {
                    self.game.set_komi(komi);
                    Komi
                }
                None       => Error
            },
            "genmove"          => {
                let color = Color::from_gtp(command[1]);
                let m  = self.engine.gen_move(color, &self.game);
                match self.game.clone().play(m) {
                    Ok(g) => {
                        self.game = g;
                        GenMove(m.to_gtp())
                    },
                    Err(_) => {
                        GenMoveError(m)
                    }
                }
            },
            "play"             => {
                let m = Move::from_gtp(command[1], command[2]);
                match self.game.clone().play(m) {
                    Ok(g) => {
                        self.game = g;
                        Play
                    },
                    Err(_) => {
                        PlayError(m)
                    }
                }
            },
            "showboard"        => ShowBoard(format!("\n{}", self.game)),
            "quit"             => return Quit,
            "final_score"      => return FinalScore(format!("{}", self.game.score())),
            _                   => return Error
        }
    }

    fn preprocess(&self, input: &str) -> String {
        let mut out = String::from_str(input);

        // We remove every control character except for LF et HT
        // the unsafe block is there because we push_byte
        unsafe {
            out = out.as_bytes().iter().fold(String::new(), |mut s, &b| if b == 9 || b == 10 || (b > 31 && b != 127) {s.push_byte(b); s} else {s});
        }

        // Then we remove anything after a #
        out = out.as_slice().split('#').next().unwrap().to_string();

        // We convert HT to SPACE (ASCII 9 to ASCII 32)
        unsafe {
            out = out.as_bytes().iter().fold(String::new(), |mut s, &b| if b == 9 {s.push_byte(32); s} else {s.push_byte(b); s});
        }

        // We remove the whitespaces before/after the string
        out = out.as_slice().trim().to_string();

        out
    }

    fn list_commands(&self) -> String {
        let mut result = String::new();

        for c in self.known_commands.iter() {
            result.push_str(c.as_slice());
            result.push_str("\n");
        }
        result.pop_char();
        result
    }
}
