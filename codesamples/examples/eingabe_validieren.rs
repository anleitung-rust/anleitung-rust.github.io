//! Beispiel: Eingabe validieren
//!
//! Dieses Programm akzeptiert nur einen einzelnen Buchstaben.

use dialog::DialogBox;

fn main() {
    loop {
        match dialog::Input::new("Gib einen Buchstaben ein:")
            .title("Buchstabeneingabe")
            .show()
        {
            Ok(Some(text)) => {
                let trimmed = text.trim();
                if trimmed.len() == 1 && trimmed.chars().all(|c| c.is_alphabetic()) {
                    println!("Dein Buchstabe: {}", trimmed);
                    break;
                } else {
                    let _ = dialog::Message::new("Bitte gib genau einen Buchstaben ein!")
                        .title("Ungültige Eingabe")
                        .show();
                }
            }
            Ok(None) => {
                println!("Abgebrochen.");
                break;
            }
            Err(e) => {
                eprintln!("Fehler: {}", e);
                break;
            }
        }
    }
}
