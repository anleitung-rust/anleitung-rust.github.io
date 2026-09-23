//! Beispiel: Ratespiel mit Schwierigkeitsgraden
//!
//! Wähle den Schwierigkeitsgrad!

use dialog::DialogBox;
use macroquad::rand::gen_range;

fn main() {
    let _ = dialog::Message::new("Willkommen zum Zahlenratespiel!\n\nWähle einen Schwierigkeitsgrad:")
        .title("Ratespiel")
        .show();
    
    let max_zahl = loop {
        match dialog::Input::new("Wähle Schwierigkeit:\n(1) Leicht (1-10)\n(2) Mittel (1-50)\n(3) Schwer (1-100)")
            .title("Schwierigkeit")
            .show()
        {
            Ok(Some(text)) => {
                match text.trim() {
                    "1" => break 11,
                    "2" => break 51,
                    "3" => break 101,
                    _ => {
                        let _ = dialog::Message::new("Bitte 1, 2 oder 3 eingeben!")
                            .title("Ungültig")
                            .show();
                    }
                }
            }
            _ => {
                println!("Abgebrochen.");
                return;
            }
        }
    };
    
    let geheime_zahl = gen_range(1, max_zahl);
    let mut versuche = 0;
    
    let _ = dialog::Message::new(&format!("Los geht's!\n\nIch habe mir eine Zahl zwischen 1 und {} ausgedacht.", max_zahl - 1))
        .title("Start")
        .show();
    
    loop {
        match dialog::Input::new(&format!("Rate die Zahl (1-{}):", max_zahl - 1))
            .title(&format!("Versuch {}", versuche + 1))
            .show()
        {
            Ok(Some(text)) => {
                match text.trim().parse::<i32>() {
                    Ok(tipp) => {
                        versuche += 1;
                        
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
                    Err(_) => {
                        let _ = dialog::Message::new("Bitte eine gültige Zahl eingeben!")
                            .title("Fehler")
                            .show();
                    }
                }
            }
            _ => {
                println!("Abgebrochen.");
                break;
            }
        }
    }
}
