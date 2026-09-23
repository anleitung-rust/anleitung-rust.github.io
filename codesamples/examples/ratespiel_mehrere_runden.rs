//! Beispiel: Ratespiel mit mehreren Runden
//!
//! Spiele mehrere Runden hintereinander!

use dialog::DialogBox;
use macroquad::rand::gen_range;

fn spiele_runde() -> u32 {
    let geheime_zahl = gen_range(0, 101);
    let mut versuche = 0;
    
    loop {
        match dialog::Input::new("Rate die Zahl (0-100):")
            .title(&format!("Versuch {}", versuche + 1))
            .show()
        {
            Ok(Some(text)) => {
                match text.trim().parse::<i32>() {
                    Ok(tipp) if tipp >= 0 && tipp <= 100 => {
                        versuche += 1;
                        
                        if tipp == geheime_zahl {
                            let _ = dialog::Message::new(&format!("Richtig!\nDie Zahl war {}.\n\nVersuche: {}", geheime_zahl, versuche))
                                .title("Gewonnen!")
                                .show();
                            return versuche;
                        } else if tipp < geheime_zahl {
                            let _ = dialog::Message::new("Zu klein!")
                                .title("Hinweis")
                                .show();
                        } else {
                            let _ = dialog::Message::new("Zu groß!")
                                .title("Hinweis")
                                .show();
                        }
                    }
                    _ => {
                        let _ = dialog::Message::new("Bitte 0-100 eingeben!")
                            .title("Ungültig")
                            .show();
                    }
                }
            }
            _ => {
                return versuche;
            }
        }
    }
}

fn main() {
    let mut gesamt_versuche = 0;
    let mut runden = 0;
    
    let _ = dialog::Message::new("Zahlenratespiel!\n\nSpiele mehrere Runden und verbessere deinen Durchschnitt!")
        .title("Start")
        .show();
    
    loop {
        runden += 1;
        
        let _ = dialog::Message::new(&format!("Runde {}", runden))
            .title("Neue Runde")
            .show();
        
        let versuche = spiele_runde();
        gesamt_versuche += versuche;
        
        let durchschnitt = gesamt_versuche as f32 / runden as f32;
        
        match dialog::Question::new(&format!("Runde {} beendet!\n\nStatistik:\n• Diese Runde: {} Versuche\n• Durchschnitt: {:.1} Versuche\n• Gesamt: {} Runden\n\nNoch eine Runde spielen?", runden, versuche, durchschnitt, runden))
            .title("Weiterspielen?")
            .show()
        {
            Ok(dialog::Choice::Yes) => {
                continue;
            }
            _ => {
                let _ = dialog::Message::new(&format!("Spiel beendet!\n\nGesamtstatistik:\n• {} Runden gespielt\n• {} Versuche insgesamt\n• {:.1} Versuche im Durchschnitt", runden, gesamt_versuche, durchschnitt))
                    .title("Endstand")
                    .show();
                break;
            }
        }
    }
}
