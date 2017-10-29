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

use std::io;
use std::io::Write;

#[derive(Debug)]
enum LainErr {
    QUIT,
    TESTERR,
}

fn lain_init() {
    println!("Initializing core modules...");
    println!("Close this world. Open the next.");
}

fn lain_eval(input: &Vec<&str>) -> Result<u32, LainErr> {
    if input.len() < 1 {
        Ok(0)
    } else {
        let command = String::from(input[0]).to_uppercase();

        // Debug atoms
        print!("({} '(", command);
        for i in 1..input.len() {
            print!("{} ", input[i]);
        }
        println!("))");
        
        match command.as_ref() {
            "EXIT"     => Err(LainErr::QUIT),
            "QUIT"     => Err(LainErr::QUIT),
            "THROWERR" => Err(LainErr::TESTERR),
            _          => Ok(0),
        }
    }
}

fn lain_repl() {
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

        let result = lain_eval(&atoms);

        match result {
            Err(LainErr::QUIT) => break,
            _                  => println!("{:?}", result),
        };
    }
}

fn lain_dispose() {
    println!("Disposing core modules...");
    //
    println!("MyLain-rs client halted. Downgrading to reality.");
}

fn main() {
    println!("MyLain-rs v0.0.0");
    println!("Hello, user!");
    lain_init();
    lain_repl();
    lain_dispose();
}
