//! Beispiel: Anzahl der Elemente
//!
//! Dieses Programm zeigt, wie man die Länge eines Vektors ermittelt.

fn main() {
    let tiere = vec!["Hund", "Katze", "Maus", "Vogel", "Fisch"];
    
    let anzahl = tiere.len();
    println!("Es gibt {} Tiere in der Liste", anzahl);
    
    // Leerer Vektor
    let leer: Vec<i32> = Vec::new();
    println!("Ein leerer Vektor hat {} Elemente", leer.len());
    
    // Prüfen, ob leer
    if leer.is_empty() {
        println!("Der Vektor ist leer!");
    }
}
