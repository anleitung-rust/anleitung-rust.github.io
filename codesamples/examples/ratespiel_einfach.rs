//! Beispiel: Einfaches Zahlenratespiel
//!
//! Ein einfaches Ratespiel für Zahlen von 0 bis 100

use dialog::DialogBox;
use macroquad::rand::gen_range;

fn main() {
    let geheime_zahl = gen_range(0, 101);
    let mut versuche = 0;
    
    let _ = dialog::Message::new("Willkommen!\n\nIch habe mir eine Zahl zwischen 0 und 100 ausgedacht.\nKannst du sie erraten?")
        .title("Zahlenratespiel")
        .show();
    
    loop {
        versuche += 1;
        
        let eingabe = match dialog::Input::new(&format!("Rate die Zahl (0-100):\nVersuch {}", versuche))
            .title("Dein Tipp")
            .show()
        {
            Ok(Some(text)) => text,
            _ => break,
        };
        
        let tipp: i32 = match eingabe.parse() {
            Ok(zahl) => zahl,
            Err(_) => {
                let _ = dialog::Message::new("Bitte gib eine gültige Zahl ein!")
                    .title("Fehler")
                    .show();
                continue;
            }
        };
        
        if tipp == geheime_zahl {
            let _ = dialog::Message::new(&format!("🎉 Richtig!\n\nDie Zahl war {}!\nDu hast {} Versuche gebraucht.", geheime_zahl, versuche))
                .title("Gewonnen!")
                .show();
            break;
        } else if tipp < geheime_zahl {
            let _ = dialog::Message::new("Zu klein! Versuche eine größere Zahl.")
                .title("Hinweis")
                .show();
        } else {
            let _ = dialog::Message::new("Zu groß! Versuche eine kleinere Zahl.")
                .title("Hinweis")
                .show();
        }
    }
    
    println!("Spiel beendet!");
}
