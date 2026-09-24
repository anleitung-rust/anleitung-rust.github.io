//! Beispiel: Einfache Eingabe - Name
//!
//! Fragt nach einem Namen.

use rustydialogs::{TextInput, TextInputMode};

fn main() {
    // ANCHOR: main
    let input = TextInput {
        title: "Name",
        message: "Wie heißt du?",
        value: "",
        mode: TextInputMode::SingleLine,
        owner: None,
    };
    match input.show() {
        Some(name) => {
            println!("Hallo, {}!", name);
        }
        _ => {
            println!("Keine Eingabe.");
        }
    };
    // ANCHOR_END: main
}
