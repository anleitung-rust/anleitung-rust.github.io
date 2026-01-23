//! Beispiel: Kombinierte Structs
//!
//! Dieses Programm zeigt Structs, die andere Structs enthalten.

struct Adresse {
    strasse: String,
    hausnummer: u32,
    stadt: String,
}

struct Person {
    name: String,
    adresse: Adresse,
}

fn main() {
    let adresse = Adresse {
        strasse: String::from("Hauptstraße"),
        hausnummer: 42,
        stadt: String::from("Berlin"),
    };
    
    let person = Person {
        name: String::from("Max"),
        adresse,
    };
    
    println!("{} wohnt in der {} {}, {}",
             person.name,
             person.adresse.strasse,
             person.adresse.hausnummer,
             person.adresse.stadt);
}
