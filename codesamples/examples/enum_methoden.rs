//! Beispiel: Enum mit Methoden
//!
//! Dieses Programm zeigt Methoden auf Enums.

enum Ampel {
    Rot,
    Gelb,
    Gruen,
}

impl Ampel {
    fn naechster(&self) -> Ampel {
        match self {
            Ampel::Rot => Ampel::Gruen,
            Ampel::Gruen => Ampel::Gelb,
            Ampel::Gelb => Ampel::Rot,
        }
    }
    
    fn als_text(&self) -> &str {
        match self {
            Ampel::Rot => "Rot (Stopp!)",
            Ampel::Gelb => "Gelb (Achtung!)",
            Ampel::Gruen => "Grün (Fahren!)",
        }
    }
}

fn main() {
    let mut ampel = Ampel::Rot;
    
    println!("Ampel: {}", ampel.als_text());
    
    ampel = ampel.naechster();
    println!("Ampel: {}", ampel.als_text());
    
    ampel = ampel.naechster();
    println!("Ampel: {}", ampel.als_text());
}
