//! Beispiel: Mehrere Eingaben
//!
//! Dieses Programm fragt nach mehreren Informationen.

use dialog::DialogBox;

fn main() {
    // Erste Eingabe: Name
    let name = match dialog::Input::new("Wie heißt du?")
        .title("Name")
        .show()
    {
        Ok(Some(n)) => n,
        _ => {
            println!("Keine Eingabe.");
            return;
        }
    };

    // Zweite Eingabe: Alter
    let alter_text = match dialog::Input::new("Wie alt bist du?")
        .title("Alter")
        .show()
    {
        Ok(Some(a)) => a,
        _ => {
            println!("Keine Eingabe.");
            return;
        }
    };

    // Alter in Zahl umwandeln
    let alter: u32 = match alter_text.parse() {
        Ok(a) => a,
        Err(_) => {
            println!("Ungültiges Alter!");
            return;
        }
    };

    // Ergebnis anzeigen
    let nachricht = format!("Hallo {}! Du bist {} Jahre alt.", name, alter);
    let _ = dialog::Message::new(&nachricht)
        .title("Deine Daten")
        .show();
}
