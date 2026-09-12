// Copyright (C) 2026 ghmaxx1k

// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 2 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License along
// with this program; if not, write to the Free Software Foundation, Inc.,
// 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.

// LICENSING
const CONDITIONS: &str = include_str!("../COPYING-initprompt/GPL-2.0.txt");
const WARRANTY: &str = include_str!("../COPYING-initprompt/GPL-nowarranty.txt");
const NOTICE: &str = include_str!("../COPYING-initprompt/GPL-notice-initprompt.txt");

// DEPENDENCIES
use std::io;
use std::io::Write;
use std::process::{Command, Stdio};

// CREATEDISK
fn createdisk() {
    
    // VARIABLES
    let mut disk_name: String = String::new();
    let mut disk_size: String = String::new();

    // PROMPT - DISK NAME AND DISK SIZE
    println!("\nvmgen> enter disk name:");
    io::stdin().read_line(&mut disk_name).expect("failed to read line");
    println!("\nvmgen> enter desired disk size (number only), (gb):");
    io::stdin().read_line(&mut disk_size).expect("failed to read line");

}

// CREATEDISK
fn createscript() {
    
    // VARIABLES
    let mut ram_size: String = String::new();
    let mut sockets: String = String::new();
    let mut cores: String = String::new();
    let mut smt: String = String::new();
    let mut iso: String = String::new();
    let mut display: String = String::new();

    // PROMPT - RAM SIZE, SOCKETS, CORES, SMT, ISO AND DISPLAY
    println!("\nvmgen> enter ram size (number only), (mb):");
    io::stdin().read_line(&mut ram_size).expect("failed to read line");
    println!("\nvmgen> enter number of sockets (1 if you only have one physical CPU):");
    io::stdin().read_line(&mut sockets).expect("failed to read line");
    println!("\nvmgen> enter number of cores (should be the same or below ur physical cores):");
    io::stdin().read_line(&mut cores).expect("failed to read line");
    println!("\nvmgen> enable smt? (2 threads per core), (yes/no):");
    io::stdin().read_line(&mut smt).expect("failed to read line");
    println!("\nvmgen> enter iso (should be on the same iso directory:");
    io::stdin().read_line(&mut iso).expect("failed to read line");
    println!("\nvmgen> enter display method (if unsure, type 'gtk'):");
    io::stdin().read_line(&mut display).expect("failed to read line");
    if smt == "yes" {
        let smt_bool = true;
    }

}

// MAIN
fn main() {
    
    // VARIABLES
    let mut user_prompt: String = String::new();

    // NOTICE AND INITIAL PROMPT
    println!("{}\n", NOTICE);
    
    // USER INPUT
    io::stdin().read_line(&mut user_prompt).expect("failed to read line");
    let user_prompt_string: &str = user_prompt.trim();
    
    // USER PROMPT MATCHER
    match user_prompt_string {
        
        // LICENSING - WARRANTY
        "show warranty" => {
            let mut pager = Command::new("less")
                .stdin(Stdio::piped())
                .spawn()
                .expect("failed to execute less")
            ;
            pager
                .stdin
                .as_mut()
                .unwrap()
                .write_all(WARRANTY.as_bytes())
                .expect("failed to write to less")
            ;
            pager.wait().expect("failed to wait for less");
        }

        // LICENSING - CONDITIONS
        "show conditions" => {
            let mut pager = Command::new("less")
                .stdin(Stdio::piped())
                .spawn()
                .expect("failed to execute less")
            ;
            pager
                .stdin
                .as_mut()
                .unwrap()
                .write_all(CONDITIONS.as_bytes())
                .expect("failed to write to less")
            ;
            pager.wait().expect("failed to wait for less");
        }

        // CREATE DISK
        "cd" => {
            createdisk();
        }

        // CREATE VIRTUAL MACHINE SCRIPT
        "cm" => {
            createscript();
        }

        // EXECUTE ALL
        "all" => {
            createscript();
            createdisk();
        }

        // NO INPUT
        _ => {}

    }

}