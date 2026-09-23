//! Beispiel: if let
//!
//! Dieses Programm zeigt if let als Alternative zu match.

enum Farbe {
    Rot,
    Gruen,
    Blau,
    RGB(u8, u8, u8),
}

fn main() {
    let farbe = Farbe::RGB(100, 150, 200);
    
    // Mit if let prüfen wir nur eine Variante
    if let Farbe::RGB(r, g, b) = farbe {
        println!("RGB-Farbe: ({}, {}, {})", r, g, b);
    } else {
        println!("Einfache Farbe");
    }
    
    let farbe2 = Farbe::Rot;
    
    if let Farbe::RGB(r, g, b) = farbe2 {
        println!("RGB: ({}, {}, {})", r, g, b);
    } else {
        println!("Keine RGB-Farbe");
    }
}
