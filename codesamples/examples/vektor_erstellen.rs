//! Beispiel: Einen Vektor erstellen
//!
//! Dieses Programm zeigt, wie man einen Vektor mit Werten erstellt.

fn main() {
    let zahlen = vec![10, 20, 30, 40, 50];
    
    println!("Mein Vektor: {:?}", zahlen);
    println!("Der Vektor hat {} Elemente", zahlen.len());
}
