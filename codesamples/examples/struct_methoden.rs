//! Beispiel: Struct mit Methoden
//!
//! Dieses Programm zeigt, wie man Methoden definiert.

struct Rechteck {
    breite: f32,
    hoehe: f32,
}

impl Rechteck {
    fn flaeche(&self) -> f32 {
        self.breite * self.hoehe
    }
    
    fn umfang(&self) -> f32 {
        2.0 * (self.breite + self.hoehe)
    }
}

fn main() {
    let rechteck = Rechteck {
        breite: 5.0,
        hoehe: 3.0,
    };
    
    println!("Rechteck: {} x {}", rechteck.breite, rechteck.hoehe);
    println!("Fläche: {}", rechteck.flaeche());
    println!("Umfang: {}", rechteck.umfang());
}
