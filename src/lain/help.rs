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

use lain::define::LainErr;
use lain::define::MYLAIN_VERSION;

pub fn lain_help(args: &[&str]) -> Result<u32, LainErr> {
    if args.len() == 0 {
        println!("MyLain-rs v{}", MYLAIN_VERSION);
        println!("The very useful, distributed personal assistant for smart homes");
        println!("This cheatsheet is temporary and may change.\n");

        println!("GENERAL COMMANDS");
        println!("help      -- () Shows this prompt.");
        println!("             (command) Shows help for said command.");
        println!("config    -- () Shows all config fields.");
        println!("quit      -- () Exits MyLain.");
        println!("exit      -- () Exits MyLain.");
        println!("throwerr  -- () Throws a test error.");

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
            "QUIT"     => println!("()         -> Exits MyLain-rs."),
            "EXIT"     => println!("()         -> Exits MyLain-rs."),
            "CONFIG"   => {
                println!("()         -> Shows all configuration fields.");
            },
            "THROWERR" => println!("()         -> Throws a test error, cathegorized under the flag TESTERR."),
            _ => {
                println!("Cannot find said command.");
                result = Err(LainErr::BADCOMMAND);
            }
        };
        result
    }
}
