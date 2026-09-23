//! Beispiel: Enum-Werte erstellen
//!
//! Dieses Programm zeigt, wie man Enum-Werte verwendet.

enum Ampel {
    Rot,
    Gelb,
    Gruen,
}

fn main() {
    let zustand1 = Ampel::Rot;
    let zustand2 = Ampel::Gruen;
    
    println!("Ampelzustände erstellt!");
}
