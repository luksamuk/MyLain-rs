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

pub const MYLAIN_VERSION: &'static str = env!("CARGO_PKG_VERSION");

// Error enumerations
#[derive(Debug)]
pub enum LainErr {
    QUIT,
    TESTERR,
    NOTIMPL,
    BADCOMMAND,
    BADCONFIG,
}

// Configuration file related
#[derive(Clone)]
pub struct LainConfig {
    pub motto: String,
    pub interface: String,
    pub port: u32,
}

impl LainConfig {
    pub fn new() -> LainConfig {
        LainConfig {
            motto: String::from(""),
            interface: String::from(""),
            port: 0,
        }
    }

    pub fn get(self, which: &str) {
        let field = String::from(which).to_uppercase();
        match field.as_ref() {
            "MOTTO"     => println!("\"{}\"", self.motto),
            "INTERFACE" => println!("{}", self.interface),
            "PORT"      => println!("{}", self.port),
            _           => println!("Unknown configuration field \"{}\"", field),
        };
    }
}

// Runtime status related
#[derive(Clone)]
pub struct LainStatus {
    
}

// Global state related
#[derive(Clone)]
pub struct LainState {
    pub config: Option<LainConfig>,
    pub status: LainStatus,
}
