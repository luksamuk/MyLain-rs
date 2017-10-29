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

const MYLAIN_VERSION: &'static str = env!("CARGO_PKG_VERSION");


#[derive(Debug)]
enum LainErr {
    QUIT,
    TESTERR,
    BADCOMMAND,
}




fn lain_help(args: &[&str]) -> Result<u32, LainErr> {
    if args.len() == 0 {
        println!("MyLain-rs v{}", MYLAIN_VERSION);
        println!("The very useful, distributed personal assistant for smart homes");
        println!("This cheatsheet is temporary and may change.\n");

        println!("GENERAL COMMANDS");
        println!("help      -- () Shows this prompt.");
        println!("             (command) Shows help for said command.");
        println!("quit      -- () Exits MyLain.");
        println!("exit      -- () Exits MyLain.");

        Ok(0)
    } else {
        println!("Showing help for command \"{}\"...", args[0]);
        let command = String::from(args[0]).to_uppercase();
        let mut result: Result<u32, LainErr> = Ok(0);
        println!("command: {}", command);
        match command.as_ref() {
            "HELP" => {
                println!("()         -> Shows a general help prompt.");
                println!("(command)  -> Shows help prompt for command.");
            },
            "QUIT" => println!("()         -> Exits MyLain-rs."),
            "EXIT" => println!("()         -> Exits MyLain-rs."),
            _ => {
                println!("Cannot find said command.");
                result = Err(LainErr::BADCOMMAND);
            }
        };
        result
    }
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
            "HELP"     => lain_help(&input[1..]),
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
    println!("MyLain-rs v{}", MYLAIN_VERSION);
    println!("Hello, user!");
    lain_init();
    lain_repl();
    lain_dispose();
}
