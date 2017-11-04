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
use lain::help;
use lain::config;

pub fn eval(input: &Vec<&str>) -> Result<u32, LainErr> {
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
            "HELP"     => help::lain_help(&input[1..]),
            "CONFIG"   => {
                let args = &input[1..];
                if args.len() == 0 {
                    config::print();
                    Ok(0)
                } else {
                    println!("Can't handle changing or viewing specific config for now, sorry.");
                    Err(LainErr::NOTIMPL)
                }
            }
            _          => Ok(0),
        }
    }
}
