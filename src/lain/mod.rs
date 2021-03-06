/******************************************************************
 *  Copyright (c) 2017 Lucas Vieira <lucasvieira@protonmail.com>  *
 *  This file is part of MyLain-rs.                               *
 *                                                                *
 *  MyLain-rs is free software: you can redistribute it and/or    *
 *  modify it under the terms of the GNU Lesser General Public    *
 *  License as published by the Free Software Foundation, either  *
 *  version 3 of the License, or (at your option) any later       *
 *  version.                                                      *
 *                                                                *
 *  You should have received a copy of the GNU Lesser General     *
 *  Public License along with MyLain-rs. If not, see              *
 *  <http://www.gnu.org/licenses/>.                               *
 ******************************************************************/

pub mod define;
pub mod repl;
pub mod help;
pub mod config;

use std::io;
use std::io::Write;
use lain::define::LainErr;
use lain::define::LainConfig;

pub fn init() -> Option<LainConfig> {
    println!("MyLain-rs v{}", define::MYLAIN_VERSION);
    println!("Hello, user!");
    println!("Initializing core modules...");
    let mut conf: Option<LainConfig> = None;
    match config::load() {
        Err(LainErr::BADCONFIG) => {
            print!("Creating config for first time... ");
            io::stdout().flush().ok();
            conf = Some(config::init());
            println!("Done.");
        },
        Err(_) => panic!("Unknown file loading error"),
        Ok(config) => {
            conf = Some(config);
            println!("MyLain config file loaded.");
        }
    };
    println!("Close this world. Open the next.");
    conf
}



pub fn dispose() {
    println!("Disposing core modules...");
    //
    println!("MyLain-rs client halted. Downgrading to reality.");
}

pub fn repl(config: &mut Option<LainConfig>) {
    loop {
        print!("lain > ");
        io::stdout().flush()
            .expect("Cannot print REPL prompt.");

        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Cannot read REPL command.");

        let input = String::from(input.trim());
        let atoms = input.split_whitespace()
            .collect::<Vec<&str>>();

        let result = repl::eval(&atoms, config);

        match result {
            Err(LainErr::QUIT) => break,
            _                  => println!("{:?}", result),
        };
    }
}
