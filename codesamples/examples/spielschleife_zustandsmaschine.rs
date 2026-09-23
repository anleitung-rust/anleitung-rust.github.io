//! Beispiel: Zustandsmaschine
//!
//! Dieses Programm verwendet einen Enum für verschiedene Spielzustände.

use dialog::DialogBox;

#[derive(Debug)]
enum Spielzustand {
    Start,
    Laeuftnoch,
    Gewonnen,
    Verloren,
}

fn main() {
    let mut zustand = Spielzustand::Start;
    let mut punkte = 0;
    
    loop {
        match zustand {
            Spielzustand::Start => {
                println!("Spiel startet...");
                let _ = dialog::Message::new("Willkommen!\n\nSammle 3 Punkte, um zu gewinnen!")
                    .title("Start")
                    .show();
                zustand = Spielzustand::Laeuftnoch;
            }
            
            Spielzustand::Laeuftnoch => {
                match dialog::Question::new(&format!("Aktuell: {} Punkte\n\nVersuche einen Punkt zu bekommen?\n(50% Chance)", punkte))
                    .title("Spielen")
                    .show()
                {
                    Ok(dialog::Choice::Yes) => {
                        let erfolg = macroquad::rand::gen_range(0, 2) == 1;
                        if erfolg {
                            punkte += 1;
                            let _ = dialog::Message::new(&format!("Erfolg! +1 Punkt\n\nJetzt: {} Punkte", punkte))
                                .title("Gewonnen!")
                                .show();
                            
                            if punkte >= 3 {
                                zustand = Spielzustand::Gewonnen;
                            }
                        } else {
                            zustand = Spielzustand::Verloren;
                        }
                    }
                    Ok(dialog::Choice::No) | Ok(dialog::Choice::Cancel) => {
                        println!("Spiel aufgegeben.");
                        let _ = dialog::Message::new(&format!("Spiel beendet.\n\nDu hattest {} Punkte.", punkte))
                            .title("Aufgegeben")
                            .show();
                        break;
                    }
                    Err(e) => {
                        eprintln!("Fehler: {}", e);
                        break;
                    }
                }
            }
            
            Spielzustand::Gewonnen => {
                println!("Spiel gewonnen!");
                let _ = dialog::Message::new("🎉 Du hast gewonnen!\n\nDu hast 3 Punkte erreicht!")
                    .title("Gewonnen!")
                    .show();
                break;
            }
            
            Spielzustand::Verloren => {
                println!("Spiel verloren!");
                let _ = dialog::Message::new(&format!("💀 Verloren!\n\nDu hattest {} Punkte.", punkte))
                    .title("Verloren")
                    .show();
                break;
            }
        }
    }
}
