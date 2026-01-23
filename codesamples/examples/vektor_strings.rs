//! Beispiel: Vektor mit Strings
//!
//! Dieses Programm zeigt, wie man einen Vektor mit Texten erstellt.

fn main() {
    let mut namen = vec![
        "Anna".to_string(),
        "Ben".to_string(),
        "Clara".to_string(),
    ];
    
    println!("Namen in der Liste:");
    for name in &namen {
        println!("- {}", name);
    }
    
    // Neuen Namen hinzufügen
    namen.push("David".to_string());
    println!("\nNach Hinzufügen:");
    for name in &namen {
        println!("- {}", name);
    }
}
