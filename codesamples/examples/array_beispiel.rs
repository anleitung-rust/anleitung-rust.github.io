//! Beispiel: Array
//!
//! Dieses Programm zeigt, wie man ein Array verwendet.

fn main() {
    // Array mit fester Größe
    let wochentage = ["Montag", "Dienstag", "Mittwoch", "Donnerstag", "Freitag"];
    
    println!("Erster Tag: {}", wochentage[0]);
    println!("Letzter Tag: {}", wochentage[4]);
    
    // Array mit Typ-Angabe
    let zahlen: [i32; 5] = [1, 2, 3, 4, 5];
    println!("\nZahlen-Array: {:?}", zahlen);
    
    // Alle Elemente durchlaufen
    println!("\nAlle Wochentage:");
    for tag in &wochentage {
        println!("- {}", tag);
    }
}
