//! Beispiel: Spielschleife mit Zustand
//!
//! Dieses Programm verwaltet einen Spielzustand.

use dialog::DialogBox;

struct Spielzustand {
    runde: u32,
    punkte: u32,
    leben: u32,
}

impl Spielzustand {
    fn neu() -> Self {
        Spielzustand {
            runde: 1,
            punkte: 0,
            leben: 3,
        }
    }
    
    fn naechste_runde(&mut self) {
        self.runde += 1;
    }
    
    fn punkt_hinzufuegen(&mut self) {
        self.punkte += 1;
    }
    
    fn leben_verlieren(&mut self) {
        if self.leben > 0 {
            self.leben -= 1;
        }
    }
}

fn main() {
    let mut spiel = Spielzustand::neu();
    
    println!("Spiel startet!");
    
    while spiel.leben > 0 && spiel.runde <= 5 {
        println!("\n=== Runde {} ===", spiel.runde);
        println!("Punkte: {} | Leben: {}", spiel.punkte, spiel.leben);
        
        match dialog::Question::new("Riskiere ein Leben für einen Punkt?")
            .title(&format!("Runde {} - Entscheidung", spiel.runde))
            .show()
        {
            Ok(dialog::Choice::Yes) => {
                spiel.punkt_hinzufuegen();
                spiel.leben_verlieren();
                let _ = dialog::Message::new(&format!("+1 Punkt!\nAber -1 Leben!\n\nJetzt: {} Punkte, {} Leben", spiel.punkte, spiel.leben))
                    .title("Ergebnis")
                    .show();
            }
            Ok(dialog::Choice::No) | Ok(dialog::Choice::Cancel) => {
                let _ = dialog::Message::new("Kein Risiko! Leben bleiben erhalten.")
                    .title("Ergebnis")
                    .show();
            }
            Err(e) => {
                eprintln!("Fehler: {}", e);
                break;
            }
        }
        
        spiel.naechste_runde();
    }
    
    let nachricht = if spiel.leben == 0 {
        format!("Game Over!\n\nDu hast {} Punkte erreicht.", spiel.punkte)
    } else {
        format!("Spiel beendet!\n\nEndergebnis: {} Punkte", spiel.punkte)
    };
    
    let _ = dialog::Message::new(&nachricht)
        .title("Spielende")
        .show();
}
