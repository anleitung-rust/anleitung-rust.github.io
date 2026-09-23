//! Beispiel: Zeichen im Text suchen
//!
//! Dieses Programm zeigt, wie man prüft, ob ein Text etwas enthält.

fn main() {
    let satz = String::from("Die Schildkröte zeichnet");
    
    if satz.contains("Schildkröte") {
        println!("Der Satz enthält das Wort 'Schildkröte'!");
    }
    
    if satz.contains("Hund") {
        println!("Der Satz enthält das Wort 'Hund'!");
    } else {
        println!("Der Satz enthält das Wort 'Hund' nicht.");
    }
    
    // Einzelne Buchstaben suchen
    if satz.contains("z") {
        println!("Der Satz enthält den Buchstaben 'z'!");
    }
}
