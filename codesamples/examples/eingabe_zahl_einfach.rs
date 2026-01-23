//! Beispiel: Einfache Zahleneingabe
//!
//! Fragt nach einer Zahl und rechnet damit.

use dialog::DialogBox;

fn main() {
    // ANCHOR: main
    match dialog::Input::new("Gib eine Zahl ein:")
        .title("Zahl")
        .show()
    {
        Ok(Some(text)) => {
            let zahl: i32 = text.parse().unwrap();  // unwrap: "Das muss eine Zahl sein!"
            println!("Deine Zahl mal 2 ist: {}", zahl * 2);
        }
        _ => {}
    }
    // ANCHOR_END: main
}
