//! Beispiel: Vektor von Structs
//!
//! Dieses Programm zeigt eine Liste von Structs.

struct Schueler {
    name: String,
    note: u32,
}

fn main() {
    let mut schueler = vec![
        Schueler {
            name: String::from("Anna"),
            note: 2,
        },
        Schueler {
            name: String::from("Ben"),
            note: 1,
        },
        Schueler {
            name: String::from("Clara"),
            note: 3,
        },
    ];
    
    println!("Klassenliste:");
    for s in &schueler {
        println!("- {}: Note {}", s.name, s.note);
    }
    
    // Neuen Schüler hinzufügen
    schueler.push(Schueler {
        name: String::from("David"),
        note: 2,
    });
    
    println!("\nAnzahl Schüler: {}", schueler.len());
}
