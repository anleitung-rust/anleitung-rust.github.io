//! Beispiel: Wortliste für Hangman
//!
//! Dieses Programm wählt ein zufälliges Wort aus einer Liste.

use macroquad::rand::gen_range;

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
    let index = gen_range(0, woerter.len());
    let geheimes_wort = woerter[index];
    
    println!("Das geheime Wort hat {} Buchstaben.", geheimes_wort.len());
    println!("(Psst, das Wort ist: {})", geheimes_wort);
}
