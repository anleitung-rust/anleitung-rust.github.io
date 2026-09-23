//! Beispiel: Match mit Enum
//!
//! Dieses Programm zeigt, wie man mit match auf Varianten reagiert.

enum Richtung {
    Oben,
    Unten,
    Links,
    Rechts,
}

fn main() {
    let richtung = Richtung::Oben;
    
    match richtung {
        Richtung::Oben => println!("Gehe nach oben!"),
        Richtung::Unten => println!("Gehe nach unten!"),
        Richtung::Links => println!("Gehe nach links!"),
        Richtung::Rechts => println!("Gehe nach rechts!"),
    }
    
    // Noch ein Beispiel
    let andere_richtung = Richtung::Links;
    
    let text = match andere_richtung {
        Richtung::Oben => "↑",
        Richtung::Unten => "↓",
        Richtung::Links => "←",
        Richtung::Rechts => "→",
    };
    
    println!("Pfeil: {}", text);
}
