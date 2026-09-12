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

fn createdisk() {

}

fn createscript() {
    
}

// MAIN
fn main() {
    
    // VARIABLES
    let mut user_prompt: String = String::new();

    // NOTICE AND INITIAL PROMPT
    println!("{}\n", NOTICE);
    
    // USER INPUT
    io::stdin().read_line(&mut user_prompt).expect("failed to read line");
    let user_prompt_string = user_prompt.trim();
    
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

        }

        // CREATE VIRTUAL MACHINE SCRIPT
        "cm" => {

        }

        // EXECUTE ALL
        "all" => {

        }

        // NO INPUT
        _ => {}

    }

}