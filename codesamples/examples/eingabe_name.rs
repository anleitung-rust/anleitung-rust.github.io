//! Beispiel: Einfache Eingabe - Name
//!
//! Fragt nach einem Namen.

use dialog::DialogBox;

fn main() {
    // ANCHOR: main
    match dialog::Input::new("Wie heißt du?")
        .title("Name")
        .show()
    {
        Ok(Some(name)) => {
            println!("Hallo, {}!", name);
        }
        _ => {
            println!("Keine Eingabe.");
        }
    }
    // ANCHOR_END: main
}
