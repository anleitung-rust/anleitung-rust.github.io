//! Beispiel: Zufälliges Element auswählen
//!
//! Dieses Programm wählt ein zufälliges Element aus einem Vektor.
//!
//! Um dieses Beispiel zu verwenden, füge in Cargo.toml hinzu:
//! [dependencies]
//! rand = "0.8"

use rand::Rng;

fn main() {
    let farben = vec!["Rot", "Grün", "Blau", "Gelb", "Orange", "Lila"];
    
    // Zufälligen Index erzeugen
    let mut rng = rand::thread_rng();
    let zufalls_index = rng.gen_range(0..farben.len());
    
    let zufaellige_farbe = &farben[zufalls_index];
    println!("Zufällig gewählte Farbe: {}", zufaellige_farbe);
}
