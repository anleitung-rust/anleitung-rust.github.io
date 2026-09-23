//! Beispiel: Struct definieren
//!
//! Dieses Programm zeigt, wie man einen Struct definiert.

struct Punkt {
    x: f32,
    y: f32,
}

fn main() {
    let p = Punkt { x: 10.0, y: 20.0 };
    println!("Punkt an Position ({}, {})", p.x, p.y);
}
