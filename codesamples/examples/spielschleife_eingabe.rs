//! Beispiel: Auf Eingaben reagieren
//!
//! Dieses Programm reagiert unterschiedlich auf verschiedene Eingaben.

use dialog::DialogBox;

fn main() {
    let mut gesundheit = 100;
    
    println!("Kampfsimulation startet!");
    
    loop {
        if gesundheit <= 0 {
            let _ = dialog::Message::new("Du wurdest besiegt!")
                .title("Game Over")
                .show();
            break;
        }
        
        match dialog::Input::new(&format!("Gesundheit: {}\n\nWas tust du?\n(a) Angreifen\n(v) Verteidigen\n(f) Fliehen", gesundheit))
            .title("Deine Aktion")
            .show()
        {
            Ok(Some(eingabe)) => {
                let aktion = eingabe.trim().to_lowercase();
                
                match aktion.as_str() {
                    "a" => {
                        let schaden = macroquad::rand::gen_range(10, 31);
                        gesundheit -= schaden;
                        let _ = dialog::Message::new(&format!("Du greifst an!\nAber der Gegner kontert!\n\n-{} Gesundheit\nVerbleibend: {}", schaden, gesundheit.max(0)))
                            .title("Angriff")
                            .show();
                    }
                    "v" => {
                        let schaden = macroquad::rand::gen_range(5, 16);
                        gesundheit -= schaden;
                        let _ = dialog::Message::new(&format!("Du verteidigst dich!\nWeniger Schaden erhalten.\n\n-{} Gesundheit\nVerbleibend: {}", schaden, gesundheit.max(0)))
                            .title("Verteidigung")
                            .show();
                    }
                    "f" => {
                        let _ = dialog::Message::new("Du fliehst aus dem Kampf!")
                            .title("Flucht")
                            .show();
                        println!("Kampf beendet durch Flucht.");
                        break;
                    }
                    _ => {
                        let _ = dialog::Message::new("Ungültige Aktion!\nWähle: (a) Angreifen, (v) Verteidigen, (f) Fliehen")
                            .title("Fehler")
                            .show();
                    }
                }
            }
            _ => {
                println!("Kampf abgebrochen.");
                break;
            }
        }
    }
    
    println!("Kampf beendet!");
}
