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

use std::io;
use std::io::Write;
use lain::define::LainErr;


pub fn init() {
    println!("MyLain-rs v{}", define::MYLAIN_VERSION);
    println!("Hello, user!");
    println!("Initializing core modules...");
    println!("Close this world. Open the next.");
}



pub fn dispose() {
    println!("Disposing core modules...");
    //
    println!("MyLain-rs client halted. Downgrading to reality.");
}

pub fn repl() {
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

        let result = repl::eval(&atoms);

        match result {
            Err(LainErr::QUIT) => break,
            _                  => println!("{:?}", result),
        };
    }
}
