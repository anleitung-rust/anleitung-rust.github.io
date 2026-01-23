//! Beispiel: Einfache Eingabe
//!
//! Dieses Programm fragt nach einem Namen.

use dialog::DialogBox;

fn main() {
    match dialog::Input::new("Wie heißt du?")
        .title("Namensabfrage")
        .show()
    {
        Ok(Some(name)) => {
            println!("Hallo, {}!", name);
        }
        Ok(None) => {
            println!("Keine Eingabe gemacht.");
        }
        Err(e) => {
            eprintln!("Fehler: {}", e);
        }
    }
}
