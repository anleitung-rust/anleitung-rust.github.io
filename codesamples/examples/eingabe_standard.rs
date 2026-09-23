//! Beispiel: Eingabe mit Standardwert
//!
//! Dieses Programm schlägt einen Standardwert vor.

use dialog::DialogBox;

fn main() {
    match dialog::Input::new("Wie heißt du?")
        .title("Name")
        .default("Max Mustermann")
        .show()
    {
        Ok(Some(name)) => {
            println!("Hallo, {}!", name);
            
            let nachricht = format!("Schön dich kennenzulernen, {}!", name);
            let _ = dialog::Message::new(&nachricht)
                .title("Begrüßung")
                .show();
        }
        Ok(None) => {
            println!("Keine Eingabe.");
        }
        Err(e) => {
            eprintln!("Fehler: {}", e);
        }
    }
}
