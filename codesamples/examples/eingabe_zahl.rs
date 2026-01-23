//! Beispiel: Zahl eingeben
//!
//! Dieses Programm fragt nach einer Zahl und wandelt sie um.

use dialog::DialogBox;

fn main() {
    match dialog::Input::new("Gib eine Zahl ein:")
        .title("Zahleneingabe")
        .show()
    {
        Ok(Some(text)) => {
            match text.parse::<i32>() {
                Ok(zahl) => {
                    println!("Du hast die Zahl {} eingegeben.", zahl);
                    println!("Das Doppelte ist: {}", zahl * 2);
                }
                Err(_) => {
                    println!("'{}' ist keine gültige Zahl!", text);
                }
            }
        }
        Ok(None) => {
            println!("Abgebrochen.");
        }
        Err(e) => {
            eprintln!("Fehler: {}", e);
        }
    }
}
