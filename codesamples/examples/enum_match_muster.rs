//! Beispiel: Match mit Mustern
//!
//! Dieses Programm extrahiert Daten aus Enum-Varianten.

enum Ereignis {
    Nachricht(String),
    Tastendruck(char),
    Klick(i32, i32),
}

fn main() {
    let ereignis1 = Ereignis::Nachricht(String::from("Hallo Welt!"));
    let ereignis2 = Ereignis::Tastendruck('a');
    let ereignis3 = Ereignis::Klick(100, 200);
    
    match ereignis1 {
        Ereignis::Nachricht(text) => {
            println!("Nachricht empfangen: {}", text);
        }
        Ereignis::Tastendruck(taste) => {
            println!("Taste gedrückt: {}", taste);
        }
        Ereignis::Klick(x, y) => {
            println!("Klick bei ({}, {})", x, y);
        }
    }
    
    match ereignis3 {
        Ereignis::Nachricht(text) => println!("Text: {}", text),
        Ereignis::Tastendruck(taste) => println!("Taste: {}", taste),
        Ereignis::Klick(x, y) => println!("Position: ({}, {})", x, y),
    }
}
