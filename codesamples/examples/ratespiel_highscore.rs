//! Beispiel: Ratespiel mit Highscore
//!
//! Das Spiel merkt sich deinen besten Score!

use dialog::DialogBox;
use macroquad::rand::gen_range;

fn main() {
    let mut highscore: Option<u32> = None;
    
    let _ = dialog::Message::new("Zahlenratespiel 0-100\n\nZiel: Erraten in möglichst wenigen Versuchen!")
        .title("Ratespiel")
        .show();
    
    let geheime_zahl = gen_range(0, 101);
    let mut versuche = 0;
    
    loop {
        match dialog::Input::new("Rate die Zahl (0-100):")
            .title(&format!("Versuch {} | Highscore: {}", versuche + 1, highscore.map_or("---".to_string(), |h| h.to_string())))
            .show()
        {
            Ok(Some(text)) => {
                match text.trim().parse::<i32>() {
                    Ok(tipp) if tipp >= 0 && tipp <= 100 => {
                        versuche += 1;
                        
                        if tipp == geheime_zahl {
                            let nachricht = if let Some(alter_highscore) = highscore {
                                if versuche < alter_highscore {
                                    highscore = Some(versuche);
                                    format!("🎉 Richtig in {} Versuchen!\n\n🏆 NEUER HIGHSCORE!\nAlter Rekord: {}", versuche, alter_highscore)
                                } else {
                                    format!("🎉 Richtig in {} Versuchen!\n\nHighscore bleibt: {}", versuche, alter_highscore)
                                }
                            } else {
                                highscore = Some(versuche);
                                format!("🎉 Richtig in {} Versuchen!\n\nDas ist dein erster Highscore!", versuche)
                            };
                            
                            let _ = dialog::Message::new(&nachricht)
                                .title("Gewonnen!")
                                .show();
                            break;
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
                        let _ = dialog::Message::new("Bitte eine Zahl zwischen 0 und 100!")
                            .title("Ungültig")
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
