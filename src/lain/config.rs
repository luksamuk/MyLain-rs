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

extern crate ini;
use lain::define::LainErr;
use lain::GLOBALCONF;

pub struct LainConfig {
    pub motto: String,
    pub port: u32,
    pub interface: String,
}

impl LainConfig {
    pub fn new() -> LainConfig {
        LainConfig {
            motto: String::from(""),
            interface: String::from(""),
            port: 0,
        }
    }
}

pub fn init() {
    let mut conf = ini::Ini::new();

    // General config
    conf.with_section(Some("General".to_owned()))
        .set("motto", "Close this world. Open the next.")
        .set("interface", "wlp3s0")
        .set("port", "6714");

    // Write
    conf.write_to_file("lain.ini")
        .unwrap();
}

pub fn load() -> Result<LainConfig, LainErr> {
    match ini::Ini::load_from_file("lain.ini") {
        Err(_) => {
            return Err(LainErr::BADCONFIG)
        },
        Ok(cfg) => {
            let s_general = cfg.section(Some("General".to_owned())).unwrap();
            let motto     = s_general.get("motto").unwrap();
            let interface = s_general.get("interface").unwrap();
            let port: u32 = s_general.get("port").unwrap().parse().unwrap();
            Ok(LainConfig {
                motto: String::from(motto.as_ref()),
                interface: String::from(interface.as_ref()),
                port: port,
            })
        }
    }
}

pub fn print() {
    println!("GENERAL CONFIGURATION");
    unsafe {
        match GLOBALCONF {
            None => println!("No configuration loaded."),
            Some(ref conf) => {
                println!("((:motto {})", conf.motto);
                println!(" (:interface {})", conf.interface);
                println!(" (:port {}))", conf.port);
            }
        };
    }
}
