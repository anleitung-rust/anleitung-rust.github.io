//! Beispiel: Text zusammenfügen
//!
//! Dieses Programm zeigt, wie man mehrere Texte kombiniert.

fn main() {
    let gruss = String::from("Hallo");
    let name = String::from("Anna");
    
    // Text mit format! zusammenfügen
    let begruessung = format!("{} {}!", gruss, name);
    println!("{}", begruessung);
    
    // Direkt ausgeben
    println!("Guten Tag, {}!", name);
}
