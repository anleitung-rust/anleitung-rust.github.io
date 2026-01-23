//! Beispiel: Eingabe verarbeiten
//!
//! Dieses Programm zeigt, wie man mit der Eingabe arbeitet.

use dialog::DialogBox;

fn main() {
    match dialog::Input::new("Gib deinen Namen ein:")
        .title("Eingabe")
        .show()
    {
        Ok(Some(eingabe)) => {
            println!("Du hast eingegeben: {}", eingabe);
            println!("Dein Name hat {} Buchstaben.", eingabe.len());
        }
        Ok(None) => {
            println!("Du hast die Eingabe abgebrochen.");
        }
        Err(e) => {
            eprintln!("Dialog-Fehler: {}", e);
        }
    }
}
