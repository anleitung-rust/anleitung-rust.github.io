//! Beispiel: Durch einen Vektor iterieren
//!
//! Dieses Programm zeigt, wie man alle Elemente durchläuft.

fn main() {
    let farben = vec!["Rot", "Grün", "Blau", "Gelb"];
    
    println!("Alle Farben:");
    for farbe in &farben {
        println!("- {}", farbe);
    }
    
    // Mit Zahlen
    let zahlen = vec![2, 4, 6, 8, 10];
    println!("\nAlle Zahlen:");
    for zahl in &zahlen {
        println!("Zahl: {}", zahl);
    }
}
