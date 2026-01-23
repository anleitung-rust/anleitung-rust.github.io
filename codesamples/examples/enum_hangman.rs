//! Beispiel: Hangman-Ergebnis
//!
//! Dieses Programm zeigt Spielergebnisse für Hangman.

enum HangmanErgebnis {
    Gewonnen,
    Verloren,
    Laeuftnoch,
}

fn pruefe_spiel(verbleibende_leben: u32, wort_komplett: bool) -> HangmanErgebnis {
    if wort_komplett {
        HangmanErgebnis::Gewonnen
    } else if verbleibende_leben == 0 {
        HangmanErgebnis::Verloren
    } else {
        HangmanErgebnis::Laeuftnoch
    }
}

fn main() {
    let ergebnis = pruefe_spiel(3, false);
    
    match ergebnis {
        HangmanErgebnis::Gewonnen => {
            println!("🎉 Gratuliere! Du hast gewonnen!");
        }
        HangmanErgebnis::Verloren => {
            println!("💀 Leider verloren. Versuch es nochmal!");
        }
        HangmanErgebnis::Laeuftnoch => {
            println!("⏳ Das Spiel läuft noch. Rate weiter!");
        }
    }
    
    // Anderes Szenario
    let ergebnis2 = pruefe_spiel(0, false);
    
    match ergebnis2 {
        HangmanErgebnis::Gewonnen => println!("Gewonnen!"),
        HangmanErgebnis::Verloren => println!("Verloren!"),
        HangmanErgebnis::Laeuftnoch => println!("Noch am spielen..."),
    }
}
