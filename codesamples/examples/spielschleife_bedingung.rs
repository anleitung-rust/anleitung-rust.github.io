//! Beispiel: Spielschleife mit Bedingung
//!
//! Dieses Programm läuft, solange Bedingungen erfüllt sind.

use dialog::DialogBox;
use macroquad::rand::gen_range;

fn main() {
    let mut leben = 3;
    let mut runde = 1;
    
    println!("Spiel startet mit {} Leben!", leben);
    
    while leben > 0 && runde <= 10 {
        println!("\n=== Runde {} ===", runde);
        
        let zufall = gen_range(1, 7); // Würfel: 1-6
        
        match dialog::Input::new(&format!("Runde {}\nRate die Würfelzahl (1-6):", runde))
            .title(&format!("Leben: {}", leben))
            .show()
        {
            Ok(Some(text)) => {
                match text.parse::<i32>() {
                    Ok(tipp) if tipp >= 1 && tipp <= 6 => {
                        if tipp == zufall {
                            let _ = dialog::Message::new(&format!("Richtig! Die Zahl war {}!\n\n✓ Leben bleibt: {}", zufall, leben))
                                .title("Gewonnen!")
                                .show();
                        } else {
                            leben -= 1;
                            let _ = dialog::Message::new(&format!("Falsch! Die Zahl war {}.\n\n✗ Leben verloren: {}", zufall, leben))
                                .title("Verloren")
                                .show();
                        }
                    }
                    _ => {
                        let _ = dialog::Message::new("Bitte eine Zahl zwischen 1 und 6 eingeben!")
                            .title("Ungültig")
                            .show();
                        continue;
                    }
                }
            }
            _ => {
                println!("Spiel abgebrochen.");
                break;
            }
        }
        
        runde += 1;
    }
    
    let nachricht = if leben == 0 {
        format!("Game Over!\n\nDu hast Runde {} erreicht.", runde - 1)
    } else {
        format!("Geschafft!\n\nDu hast alle {} Runden überlebt!", runde - 1)
    };
    
    let _ = dialog::Message::new(&nachricht)
        .title("Spielende")
        .show();
}
