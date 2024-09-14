use colored::*;
use inputbot::KeybdKey::EKey;
use std::thread;
use std::time::Duration;

fn main() {
    println!("5 seconds to tab into Roblox");
    thread::sleep(Duration::from_secs(5));

    loop {
        wainbowzzzzz("Clicking");
        EKey.press();
        EKey.release();
        wainbowzzzzz("Sleeping");
        thread::sleep(Duration::from_secs(25));
    }
}

fn wainbowzzzzz(elwordoz: &str) {
    let dawainbow = ["red", "yellow", "green", "cyan", "blue", "magenta"];
    for (i, c) in elwordoz.chars().enumerate() {
        print!("{}", c.to_string().color(dawainbow[i % dawainbow.len()]));
    }
    println!();
}