// SPDX-License-Identifier: GPL-3.0-or-later

extern crate ansi_term;
use ansi_term::Colour::{Blue, Red, Yellow};

fn main() {
    println!("Hello, world!");
    println!("This is in red: {}!", Red.paint("a red string"));
    println!(
        "Demonstrating {} and {}!",
        Blue.on(Yellow).paint("Blue on yellow!"),
        Yellow.underline().paint("yellow underline")
    );
}
