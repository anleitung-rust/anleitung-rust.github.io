//! Beispiel: Textlänge
//!
//! Dieses Programm zeigt, wie man die Länge eines Textes ermittelt.

fn main() {
    let wort = String::from("Schildkröte");
    let anzahl = wort.len();
    
    println!("Das Wort '{}' hat {} Zeichen.", wort, anzahl);
    
    // Verschiedene Wörter
    let kurz = String::from("Hi");
    let lang = String::from("Programmieren");
    
    println!("'{}' hat {} Zeichen", kurz, kurz.len());
    println!("'{}' hat {} Zeichen", lang, lang.len());
}
