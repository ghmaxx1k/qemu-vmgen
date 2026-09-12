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

// DEPENDENCIES
use std::io;
use std::io::Write;

// CREATEDISK
pub fn createdisk() {
    
    // VARIABLES
    let mut disk_name: String = String::new();
    let mut disk_size: String = String::new();

    // PROMPT - DISK NAME AND DISK SIZE
    print!("\ndisk > enter disk name: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut disk_name).expect("failed to read line");
    print!("\ndisk > enter desired disk size (number only), (gb): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut disk_size).expect("failed to read line");

}

// CREATEDISK
pub fn createscript() {
    
    // VARIABLES
    let mut ram_size: String = String::new();
    let mut sockets: String = String::new();
    let mut cores: String = String::new();
    let mut smt: String = String::new();
    let mut iso: String = String::new();
    let mut display: String = String::new();

    // PROMPT - RAM SIZE, SOCKETS, CORES, SMT, ISO AND DISPLAY
    print!("\nram > enter ram size (number only), (mb): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut ram_size).expect("failed to read line");
    print!("\ncpu > enter number of sockets (1 if you only have one physical CPU): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut sockets).expect("failed to read line");
    print!("\ncpu > enter number of cores (should be the same or below ur physical cores): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut cores).expect("failed to read line");
    print!("\ncpu > enable smt? (2 threads per core), (yes/no): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut smt).expect("failed to read line");
    print!("\nboot > enter iso (should be on the same iso directory: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut iso).expect("failed to read line");
    print!("\nrender > enter display method (if unsure, type 'gtk'): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut display).expect("failed to read line");
    if smt == "yes" {
        let smt_bool = true;
    }

}