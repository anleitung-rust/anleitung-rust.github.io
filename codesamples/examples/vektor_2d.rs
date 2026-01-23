//! Beispiel: Zweidimensionaler Vektor
//!
//! Dieses Programm zeigt eine Liste von Listen.

fn main() {
    let spielfeld = vec![
        vec![1, 0, 0],
        vec![0, 1, 0],
        vec![0, 0, 1],
    ];
    
    println!("Spielfeld (3x3):");
    for zeile in &spielfeld {
        println!("{:?}", zeile);
    }
    
    // Auf einzelnes Element zugreifen
    println!("\nElement in Zeile 1, Spalte 2: {}", spielfeld[1][2]);
}
