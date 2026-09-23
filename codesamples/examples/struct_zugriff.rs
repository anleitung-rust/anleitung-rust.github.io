//! Beispiel: Auf Felder zugreifen
//!
//! Dieses Programm zeigt, wie man Felder ausliest.

struct Buch {
    titel: String,
    autor: String,
    seiten: u32,
}

fn main() {
    let buch = Buch {
        titel: String::from("Rust lernen"),
        autor: String::from("Max Muster"),
        seiten: 200,
    };
    
    println!("Titel: {}", buch.titel);
    println!("Autor: {}", buch.autor);
    println!("Seiten: {}", buch.seiten);
}
