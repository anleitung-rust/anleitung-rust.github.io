//! Beispiel: Spieler-Struct
//!
//! Dieses Programm zeigt einen Struct für einen Spieler.

struct Spieler {
    name: String,
    punkte: u32,
    leben: u32,
}

impl Spieler {
    fn ist_am_leben(&self) -> bool {
        self.leben > 0
    }
    
    fn punkte_hinzufuegen(&mut self, punkte: u32) {
        self.punkte += punkte;
    }
    
    fn schaden_nehmen(&mut self, schaden: u32) {
        if self.leben >= schaden {
            self.leben -= schaden;
        } else {
            self.leben = 0;
        }
    }
}

fn main() {
    let mut spieler = Spieler {
        name: String::from("Anna"),
        punkte: 0,
        leben: 3,
    };
    
    println!("{} startet mit {} Leben", spieler.name, spieler.leben);
    
    spieler.punkte_hinzufuegen(10);
    println!("Punkte gesammelt! Jetzt: {} Punkte", spieler.punkte);
    
    spieler.schaden_nehmen(1);
    println!("Schaden genommen! Leben: {}", spieler.leben);
    
    if spieler.ist_am_leben() {
        println!("{} lebt noch!", spieler.name);
    }
}
