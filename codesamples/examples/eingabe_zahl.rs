//! Beispiel: Zahl eingeben
//!
//! Dieses Programm fragt nach einer Zahl und wandelt sie um.

use dialog::DialogBox;
use turtle_lib::turtle_main;

#[turtle_main]
fn main() {
    match dialog::Input::new("Gib eine Zahl ein:")
        .title("Zahleneingabe")
        .show()
    {
        Ok(Some(text)) => match text.parse::<i32>() {
            Ok(zahl) => {
                turtle.write_text(&format!("Du hast die Zahl {} eingegeben.", zahl), 50.0);
                turtle.go_to((0.0, 50.0));
                turtle.write_text(&format!("Das Doppelte ist: {}", zahl * 2), 50.0);
            }
            Err(_) => {
                turtle.write_text(&format!("'{}' ist keine gültige Zahl!", text), 50.0);
            }
        },
        Ok(None) => {
            turtle.write_text("Abgebrochen.", 50.0);
        }
        Err(e) => {
            turtle.write_text(&format!("Fehler: {}", e), 50.0);
        }
    }
}
