//! Beispiel: Groß- und Kleinschreibung
//!
//! Dieses Programm zeigt, wie man Text umwandelt.

fn main() {
    let text = String::from("Hallo Welt");
    
    println!("Original: {}", text);
    println!("Großbuchstaben: {}", text.to_uppercase());
    println!("Kleinbuchstaben: {}", text.to_lowercase());
    
    // Vergleich ohne Groß-/Kleinschreibung
    let eingabe = String::from("APFEL");
    let richtig = String::from("apfel");
    
    if eingabe.to_lowercase() == richtig {
        println!("Richtig geraten! (ohne Beachtung der Groß-/Kleinschreibung)");
    }
}
