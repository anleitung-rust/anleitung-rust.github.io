//! Beispiel: Zahlenratespiel Schritt-für-Schritt
//!
//! Vollständiges einfaches Ratespiel mit allen Schritten.

use std::time::{SystemTime, UNIX_EPOCH};

use macroquad::rand::gen_range;
use rustydialogs::{TextInput, TextInputMode};

fn main() {
    macroquad::rand::srand(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64,
    );

    // ANCHOR: step1
    let geheime_zahl = gen_range(0, 101); // 0 bis 100
    println!("Ich habe mir eine Zahl ausgedacht!");
    // ANCHOR_END: step1

    // ANCHOR: step2
    let eingabe = TextInput {
        title: "Ratespiel",
        message: "Rate die Zahl (0-100):",
        value: "",
        mode: TextInputMode::SingleLine,
        owner: None,
    }
    .show()
    .unwrap();

    let tipp: i32 = eingabe.parse().unwrap();

    println!("Du hast {} geraten", tipp);
    // ANCHOR_END: step2

    // ANCHOR: step3
    if tipp == geheime_zahl {
        println!("Richtig!");
    } else if tipp < geheime_zahl {
        println!("Zu klein!");
    } else {
        println!("Zu groß!");
    }
    // ANCHOR_END: step3
}
