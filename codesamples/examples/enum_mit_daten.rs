//! Beispiel: Enum mit Daten
//!
//! Dieses Programm zeigt Enums, die Daten enthalten.

enum Nachricht {
    Text(String),
    Zahl(i32),
    Position(f32, f32),
}

fn main() {
    let msg1 = Nachricht::Text(String::from("Hallo!"));
    let msg2 = Nachricht::Zahl(42);
    let msg3 = Nachricht::Position(10.5, 20.3);
    
    match msg1 {
        Nachricht::Text(t) => println!("Text-Nachricht: {}", t),
        Nachricht::Zahl(n) => println!("Zahl-Nachricht: {}", n),
        Nachricht::Position(x, y) => println!("Position: ({}, {})", x, y),
    }
    
    match msg2 {
        Nachricht::Text(t) => println!("Text: {}", t),
        Nachricht::Zahl(n) => println!("Zahl: {}", n),
        Nachricht::Position(x, y) => println!("Pos: ({}, {})", x, y),
    }
}
