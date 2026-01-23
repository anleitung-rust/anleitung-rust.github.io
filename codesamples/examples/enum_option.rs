//! Beispiel: Option Enum
//!
//! Dieses Programm zeigt die Verwendung von Option.

fn finde_zahl(zahlen: &Vec<i32>, suche: i32) -> Option<usize> {
    for (index, &zahl) in zahlen.iter().enumerate() {
        if zahl == suche {
            return Some(index);
        }
    }
    None
}

fn main() {
    let zahlen = vec![10, 20, 30, 40, 50];
    
    let ergebnis = finde_zahl(&zahlen, 30);
    
    match ergebnis {
        Some(index) => println!("Gefunden an Position {}", index),
        None => println!("Nicht gefunden"),
    }
    
    let ergebnis2 = finde_zahl(&zahlen, 99);
    
    match ergebnis2 {
        Some(index) => println!("Gefunden an Position {}", index),
        None => println!("99 ist nicht in der Liste"),
    }
}
