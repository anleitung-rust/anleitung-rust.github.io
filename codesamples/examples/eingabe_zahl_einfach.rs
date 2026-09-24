//! Beispiel: Einfache Zahleneingabe
//!
//! Fragt nach einer Zahl und rechnet damit.

use rustydialogs::{MessageBox, MessageButtons, MessageIcon, TextInput, TextInputMode};

fn main() {
    // ANCHOR: main
    let input = TextInput {
        title: "Zahl",
        message: "Gib eine Zahl ein:",
        value: "",
        mode: TextInputMode::SingleLine,
        owner: None,
    };

    match input.show() {
        Some(text) => match text.parse::<i32>() {
            Ok(zahl) => {
                let message = format!("Deine Zahl mal 2 ist: {}", zahl * 2);
                let _ = MessageBox {
                    title: "Ergebnis",
                    message: &message,
                    icon: MessageIcon::Info,
                    buttons: MessageButtons::Ok,
                    owner: None,
                }
                .show();
            }
            Err(_) => {
                let _ = MessageBox {
                    title: "Fehler",
                    message: "Das muss eine Zahl sein!",
                    icon: MessageIcon::Warning,
                    buttons: MessageButtons::Ok,
                    owner: None,
                }
                .show();
            }
        },
        None => {
            let _ = MessageBox {
                title: "Abbruch",
                message: "Keine Eingabe.",
                icon: MessageIcon::Info,
                buttons: MessageButtons::Ok,
                owner: None,
            }
            .show();
        }
    }
    // ANCHOR_END: main
}
