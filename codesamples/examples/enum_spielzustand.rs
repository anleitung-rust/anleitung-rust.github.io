//! Beispiel: Spielzustand mit Enum
//!
//! Dieses Programm modelliert Spielzustände.

enum Spielzustand {
    Startmenue,
    Laedt,
    Laeuft,
    Pause,
    Gewonnen,
    Verloren,
}

fn main() {
    let mut zustand = Spielzustand::Startmenue;
    
    match zustand {
        Spielzustand::Startmenue => println!("Drücke Start!"),
        Spielzustand::Laedt => println!("Lade..."),
        Spielzustand::Laeuft => println!("Spiel läuft!"),
        Spielzustand::Pause => println!("Pausiert"),
        Spielzustand::Gewonnen => println!("Du hast gewonnen! 🎉"),
        Spielzustand::Verloren => println!("Verloren. Versuch es nochmal!"),
    }
    
    // Zustand ändern
    zustand = Spielzustand::Laeuft;
    
    match zustand {
        Spielzustand::Laeuft => println!("Das Spiel läuft jetzt!"),
        _ => println!("Anderer Zustand"),
    }
}
