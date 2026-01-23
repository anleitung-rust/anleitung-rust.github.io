//! Beispiel: Wortliste für Hangman
//!
//! Dieses Programm wählt ein zufälliges Wort aus einer Liste.
//!
//! Um dieses Beispiel zu verwenden, füge in Cargo.toml hinzu:
//! [dependencies]
//! rand = "0.8"

use rand::Rng;

fn main() {
    let woerter = vec![
        "Schildkröte",
        "Programmieren",
        "Computer",
        "Spiel",
        "Farbe",
        "Bewegung",
        "Zeichnen",
    ];
    
    println!("Willkommen beim Hangman!");
    println!("Wir haben {} Wörter zur Auswahl.", woerter.len());
    
    // Zufälliges Wort auswählen
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..woerter.len());
    let geheimes_wort = woerter[index];
    
    println!("Das geheime Wort hat {} Buchstaben.", geheimes_wort.len());
    println!("(Psst, das Wort ist: {})", geheimes_wort);
}
