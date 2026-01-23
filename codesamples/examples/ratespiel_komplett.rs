//! Beispiel: Vollständiges Zahlenratespiel 0-100
//!
//! Rate die Zahl zwischen 0 und 100!

use dialog::DialogBox;
use macroquad::rand::gen_range;

struct Ratespiel {
    geheime_zahl: i32,
    versuche: u32,
    laeuftnoch: bool,
}

impl Ratespiel {
    fn neu() -> Self {
        Ratespiel {
            geheime_zahl: gen_range(0, 101),
            versuche: 0,
            laeuftnoch: true,
        }
    }
    
    fn rate(&mut self, tipp: i32) -> String {
        self.versuche += 1;
        
        if tipp == self.geheime_zahl {
            self.laeuftnoch = false;
            format!("🎉 Richtig!\n\nDie Zahl war {}!\nDu hast {} Versuche gebraucht.", self.geheime_zahl, self.versuche)
        } else if tipp < self.geheime_zahl {
            format!("Zu klein!\n\nDie gesuchte Zahl ist größer als {}.\nVersuch: {}", tipp, self.versuche)
        } else {
            format!("Zu groß!\n\nDie gesuchte Zahl ist kleiner als {}.\nVersuch: {}", tipp, self.versuche)
        }
    }
}

fn main() {
    let mut spiel = Ratespiel::neu();
    
    println!("Zahlenratespiel 0-100 gestartet!");
    
    let _ = dialog::Message::new("Willkommen zum Zahlenratespiel!\n\nIch habe mir eine Zahl zwischen 0 und 100 ausgedacht.\nKannst du sie erraten?")
        .title("Ratespiel")
        .show();
    
    while spiel.laeuftnoch {
        match dialog::Input::new("Rate die Zahl (0-100):")
            .title(&format!("Versuch {}", spiel.versuche + 1))
            .show()
        {
            Ok(Some(text)) => {
                match text.trim().parse::<i32>() {
                    Ok(tipp) if tipp >= 0 && tipp <= 100 => {
                        let ergebnis = spiel.rate(tipp);
                        let _ = dialog::Message::new(&ergebnis)
                            .title("Ergebnis")
                            .show();
                    }
                    Ok(_) => {
                        let _ = dialog::Message::new("Bitte eine Zahl zwischen 0 und 100 eingeben!")
                            .title("Ungültig")
                            .show();
                    }
                    Err(_) => {
                        let _ = dialog::Message::new("Das ist keine gültige Zahl!\nBitte versuche es erneut.")
                            .title("Fehler")
                            .show();
                    }
                }
            }
            Ok(None) => {
                println!("Spiel abgebrochen.");
                let _ = dialog::Message::new(&format!("Spiel abgebrochen.\n\nDie Zahl war: {}", spiel.geheime_zahl))
                    .title("Abgebrochen")
                    .show();
                break;
            }
            Err(e) => {
                eprintln!("Dialog-Fehler: {}", e);
                break;
            }
        }
    }
    
    println!("Spiel beendet!");
}
