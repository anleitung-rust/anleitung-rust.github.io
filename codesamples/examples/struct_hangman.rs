//! Beispiel: Hangman-Spielzustand
//!
//! Dieses Programm zeigt einen Struct für Hangman.

struct HangmanSpiel {
    geheimes_wort: String,
    geratene_buchstaben: Vec<char>,
    fehler: u32,
    max_fehler: u32,
}

impl HangmanSpiel {
    fn neu(wort: String) -> HangmanSpiel {
        HangmanSpiel {
            geheimes_wort: wort,
            geratene_buchstaben: Vec::new(),
            fehler: 0,
            max_fehler: 6,
        }
    }
    
    fn buchstabe_raten(&mut self, buchstabe: char) {
        self.geratene_buchstaben.push(buchstabe);
        
        if !self.geheimes_wort.contains(buchstabe) {
            self.fehler += 1;
        }
    }
    
    fn ist_verloren(&self) -> bool {
        self.fehler >= self.max_fehler
    }
}

fn main() {
    let mut spiel = HangmanSpiel::neu(String::from("Schildkröte"));
    
    println!("Hangman-Spiel gestartet!");
    println!("Maximale Fehler: {}", spiel.max_fehler);
    
    spiel.buchstabe_raten('e');
    println!("Buchstabe 'e' geraten. Fehler: {}", spiel.fehler);
    
    spiel.buchstabe_raten('x');
    println!("Buchstabe 'x' geraten. Fehler: {}", spiel.fehler);
    
    if spiel.ist_verloren() {
        println!("Verloren!");
    } else {
        println!("Noch am Leben!");
    }
}
