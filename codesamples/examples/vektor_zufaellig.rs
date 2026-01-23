//! Beispiel: Zufälliges Element auswählen
//!
//! Dieses Programm wählt ein zufälliges Element aus einem Vektor.

use macroquad::rand::gen_range;

fn main() {
    let farben = vec!["Rot", "Grün", "Blau", "Gelb", "Orange", "Lila"];
    
    // Zufälligen Index erzeugen
    let zufalls_index = gen_range(0, farben.len());
    
    let zufaellige_farbe = &farben[zufalls_index];
    println!("Zufällig gewählte Farbe: {}", zufaellige_farbe);
}
