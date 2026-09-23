//! Beispiel: Robuste Zahleneingabe
//!
//! Diese Funktion fragt so lange, bis eine gültige Zahl eingegeben wurde.

use dialog::DialogBox;

fn frage_nach_zahl(frage: &str) -> i32 {
    loop {
        match dialog::Input::new(frage)
            .title("Zahleneingabe")
            .show()
        {
            Ok(Some(text)) => {
                match text.parse::<i32>() {
                    Ok(zahl) => return zahl,
                    Err(_) => {
                        let _ = dialog::Message::new("Das ist keine gültige Zahl!\nBitte versuche es erneut.")
                            .title("Fehler")
                            .show();
                    }
                }
            }
            Ok(None) => {
                let _ = dialog::Message::new("Eingabe ist erforderlich!")
                    .title("Fehler")
                    .show();
            }
            Err(e) => {
                eprintln!("Dialog-Fehler: {}", e);
                std::process::exit(1);
            }
        }
    }
}

fn main() {
    let zahl = frage_nach_zahl("Gib eine Zahl ein:");
    println!("Du hast die Zahl {} eingegeben.", zahl);
    
    let nachricht = format!("Deine Zahl: {}\nDas Doppelte: {}", zahl, zahl * 2);
    let _ = dialog::Message::new(&nachricht)
        .title("Ergebnis")
        .show();
}
