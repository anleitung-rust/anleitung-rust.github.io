//! Beispiel: Felder ändern
//!
//! Dieses Programm zeigt, wie man Felder ändert.

struct Zaehler {
    wert: u32,
}

fn main() {
    let mut zaehler = Zaehler { wert: 0 };
    
    println!("Start: {}", zaehler.wert);
    
    zaehler.wert += 1;
    println!("Nach +1: {}", zaehler.wert);
    
    zaehler.wert += 5;
    println!("Nach +5: {}", zaehler.wert);
}
