//! Beispiel: Ja/Nein-Frage
//!
//! Dieses Programm stellt eine Ja/Nein-Frage.

use dialog::DialogBox;

fn main() {
    match dialog::Question::new("Möchtest du fortfahren?")
        .title("Bestätigung")
        .show()
    {
        Ok(dialog::Choice::Yes) => {
            println!("Du hast JA gewählt!");
            let _ = dialog::Message::new("Wir fahren fort...")
                .title("Fortfahren")
                .show();
        }
        Ok(dialog::Choice::No) | Ok(dialog::Choice::Cancel) => {
            println!("Du hast NEIN gewählt!");
            let _ = dialog::Message::new("Programm wird beendet.")
                .title("Beenden")
                .show();
        }
        Err(e) => {
            eprintln!("Fehler: {}", e);
        }
    }
}
