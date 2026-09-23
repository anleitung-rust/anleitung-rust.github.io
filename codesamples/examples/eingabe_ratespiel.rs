//! Beispiel: Einfaches Ratespiel
//!
//! Der Computer wählt eine Zahl, und du musst sie erraten.

use dialog::DialogBox;
use macroquad::rand::gen_range;

fn main() {
    // Zufällige Zahl zwischen 1 und 10
    let geheime_zahl = gen_range(1, 11);
    
    println!("Ich habe mir eine Zahl zwischen 1 und 10 ausgedacht!");
    
    let _ = dialog::Message::new("Ich habe mir eine Zahl zwischen 1 und 10 ausgedacht!\n\nKannst du sie erraten?")
        .title("Ratespiel")
        .show();
    
    loop {
        match dialog::Input::new("Rate die Zahl (1-10):")
            .title("Dein Tipp")
            .show()
        {
            Ok(Some(text)) => {
                match text.parse::<i32>() {
                    Ok(tipp) => {
                        if tipp == geheime_zahl {
                            let _ = dialog::Message::new(&format!("Richtig! Die Zahl war {}!\n\n🎉 Du hast gewonnen!", geheime_zahl))
                                .title("Gewonnen!")
                                .show();
                            println!("Spiel gewonnen!");
                            break;
                        } else if tipp < geheime_zahl {
                            let _ = dialog::Message::new("Zu klein! Versuche eine größere Zahl.")
                                .title("Falsch")
                                .show();
                        } else {
                            let _ = dialog::Message::new("Zu groß! Versuche eine kleinere Zahl.")
                                .title("Falsch")
                                .show();
                        }
                    }
                    Err(_) => {
                        let _ = dialog::Message::new("Bitte gib eine gültige Zahl ein!")
                            .title("Fehler")
                            .show();
                    }
                }
            }
            Ok(None) => {
                println!("Spiel abgebrochen.");
                break;
            }
            Err(e) => {
                eprintln!("Fehler: {}", e);
                break;
            }
        }
    }
}
