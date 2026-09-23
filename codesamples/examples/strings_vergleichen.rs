//! Beispiel: Text vergleichen
//!
//! Dieses Programm zeigt, wie man Texte vergleicht.

fn main() {
    let wort1 = String::from("Katze");
    let wort2 = String::from("Katze");
    let wort3 = String::from("Hund");
    
    if wort1 == wort2 {
        println!("'{}' und '{}' sind gleich!", wort1, wort2);
    }
    
    if wort1 != wort3 {
        println!("'{}' und '{}' sind verschieden!", wort1, wort3);
    }
    
    // Groß-/Kleinschreibung beachten
    let gross = String::from("HALLO");
    let klein = String::from("hallo");
    
    if gross != klein {
        println!("'{}' und '{}' sind nicht gleich (Groß-/Kleinschreibung!)", gross, klein);
    }
}
