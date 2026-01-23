//! Beispiel: Einfache Spielschleife
//!
//! Dieses Programm zeigt eine grundlegende Spielschleife.

use dialog::DialogBox;

fn main() {
    println!("Spiel startet!");
    
    for runde in 1..=5 {
        println!("\n=== Runde {} ===", runde);
        
        let _ = dialog::Message::new(&format!("Runde {} von 5", runde))
            .title("Spielfortschritt")
            .show();
        
        match dialog::Input::new("Drücke OK für die nächste Runde")
            .title("Weiter")
            .show()
        {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Fehler: {}", e);
                break;
            }
        }
    }
    
    println!("\nSpiel beendet!");
    let _ = dialog::Message::new("Spiel vorbei! Danke fürs Spielen!")
        .title("Ende")
        .show();
}
