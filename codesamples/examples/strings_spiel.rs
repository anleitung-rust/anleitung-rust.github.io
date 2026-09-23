//! Beispiel: Geheimes Wort für ein Spiel
//!
//! Dieses Programm simuliert ein einfaches Ratespiel.

fn main() {
    // Das geheime Wort
    let geheimes_wort = String::from("Schildkröte");
    
    println!("Willkommen beim Ratespiel!");
    println!("Das geheime Wort hat {} Buchstaben.", geheimes_wort.len());
    
    // Simuliere eine Rateeingabe
    let versuch1 = String::from("Hund");
    let versuch2 = String::from("Schildkröte");
    
    // Erster Versuch
    if versuch1 == geheimes_wort {
        println!("'{}' ist richtig! Gewonnen!", versuch1);
    } else {
        println!("'{}' ist leider falsch.", versuch1);
    }
    
    // Zweiter Versuch
    if versuch2 == geheimes_wort {
        println!("'{}' ist richtig! Gewonnen!", versuch2);
    } else {
        println!("'{}' ist leider falsch.", versuch2);
    }
}
