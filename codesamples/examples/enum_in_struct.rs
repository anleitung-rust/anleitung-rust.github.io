//! Beispiel: Enum in Struct
//!
//! Dieses Programm kombiniert Struct und Enum.

enum Status {
    Aktiv,
    Inaktiv,
    Gesperrt,
}

struct Benutzer {
    name: String,
    status: Status,
}

impl Benutzer {
    fn kann_einloggen(&self) -> bool {
        match self.status {
            Status::Aktiv => true,
            Status::Inaktiv | Status::Gesperrt => false,
        }
    }
}

fn main() {
    let benutzer1 = Benutzer {
        name: String::from("Anna"),
        status: Status::Aktiv,
    };
    
    let benutzer2 = Benutzer {
        name: String::from("Ben"),
        status: Status::Gesperrt,
    };
    
    if benutzer1.kann_einloggen() {
        println!("{} kann sich einloggen!", benutzer1.name);
    } else {
        println!("{} kann sich nicht einloggen.", benutzer1.name);
    }
    
    if benutzer2.kann_einloggen() {
        println!("{} kann sich einloggen!", benutzer2.name);
    } else {
        println!("{} kann sich nicht einloggen.", benutzer2.name);
    }
}
