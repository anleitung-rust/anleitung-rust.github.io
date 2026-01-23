//! Beispiel: String ändern
//!
//! Dieses Programm zeigt, wie man einen String erweitert.

fn main() {
    let mut text = String::from("Hallo");
    println!("Vorher: {}", text);
    
    text.push_str(" Welt");
    println!("Nachher: {}", text);
    
    // Noch mehr hinzufügen
    text.push_str("! Wie geht es dir?");
    println!("Am Ende: {}", text);
}
