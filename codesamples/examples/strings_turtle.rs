//! Beispiel: Text mit Turtle
//!
//! Zeigt, wie man Text mit der Turtle kombiniert.

use turtle_lib::*;

#[turtle_main]
fn main() {
    // ANCHOR: main
    let name = "Max";
    println!("Hallo, {}!", name);

    turtle.write_text(name, 50.0);
    // ANCHOR_END: main
}
