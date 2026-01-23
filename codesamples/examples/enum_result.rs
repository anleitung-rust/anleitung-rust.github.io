//! Beispiel: Result Enum
//!
//! Dieses Programm zeigt die Verwendung von Result.

fn teile(a: f32, b: f32) -> Result<f32, String> {
    if b == 0.0 {
        Err(String::from("Division durch Null!"))
    } else {
        Ok(a / b)
    }
}

fn main() {
    let ergebnis1 = teile(10.0, 2.0);
    
    match ergebnis1 {
        Ok(wert) => println!("Ergebnis: {}", wert),
        Err(fehler) => println!("Fehler: {}", fehler),
    }
    
    let ergebnis2 = teile(5.0, 0.0);
    
    match ergebnis2 {
        Ok(wert) => println!("Ergebnis: {}", wert),
        Err(fehler) => println!("Fehler: {}", fehler),
    }
}
