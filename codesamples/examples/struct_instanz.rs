//! Beispiel: Struct-Instanz erstellen
//!
//! Dieses Programm zeigt, wie man eine Instanz erstellt.

struct Person {
    name: String,
    alter: u32,
    stadt: String,
}

fn main() {
    let person = Person {
        name: String::from("Anna"),
        alter: 12,
        stadt: String::from("Berlin"),
    };
    
    println!("Person erstellt: {} aus {}, {} Jahre alt", 
             person.name, person.stadt, person.alter);
}
