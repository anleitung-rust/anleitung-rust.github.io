//! Beispiel: Enum definieren
//!
//! Dieses Programm zeigt, wie man einen Enum definiert.

enum Richtung {
    Oben,
    Unten,
    Links,
    Rechts,
}

fn main() {
    let bewegung = Richtung::Oben;
    println!("Richtung gewählt!");
}
