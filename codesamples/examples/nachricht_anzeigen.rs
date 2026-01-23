//! Beispiel: Nachricht anzeigen
//!
//! Dieses Programm zeigt eine einfache Nachricht an.

use dialog::DialogBox;

fn main() {
    println!("Programm gestartet!");
    
    let _ = dialog::Message::new("Willkommen zu meinem Programm!\n\nDies ist eine einfache Nachricht.")
        .title("Willkommen")
        .show();
    
    println!("Nachricht wurde angezeigt.");
    
    let _ = dialog::Message::new("Das Programm ist jetzt fertig. Auf Wiedersehen!")
        .title("Fertig")
        .show();
}
