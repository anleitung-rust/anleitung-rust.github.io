//! Beispiel: Einfaches Zahlenratespiel
//!
//! Ein einfaches Ratespiel für Zahlen von 0 bis 100

use std::time::{SystemTime, UNIX_EPOCH};

use macroquad::rand::gen_range;
use rustydialogs::{MessageBox, MessageButtons, MessageIcon, TextInput, TextInputMode};

fn main() {
    macroquad::rand::srand(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64,
    );

    let geheime_zahl = gen_range(0, 101);
    let mut versuche = 0;

    let welcome = "Willkommen!\n\nIch habe mir eine Zahl zwischen 0 und 100 ausgedacht.\nKannst du sie erraten?";
    let _ = MessageBox {
        title: "Zahlenratespiel",
        message: welcome,
        icon: MessageIcon::Info,
        buttons: MessageButtons::Ok,
        owner: None,
    }
    .show();

    loop {
        versuche += 1;

        let prompt = format!("Rate die Zahl (0-100):\nVersuch {}", versuche);
        let eingabe = match (TextInput {
            title: "Dein Tipp",
            message: &prompt,
            value: "",
            mode: TextInputMode::SingleLine,
            owner: None,
        })
        .show()
        {
            Some(text) => text,
            None => break,
        };

        let tipp: i32 = match eingabe.parse() {
            Ok(zahl) => zahl,
            Err(_) => {
                let message = "Bitte gib eine gültige Zahl ein!";
                let _ = MessageBox {
                    title: "Fehler",
                    message,
                    icon: MessageIcon::Warning,
                    buttons: MessageButtons::Ok,
                    owner: None,
                }
                .show();
                continue;
            }
        };

        // ANCHOR: richtig_nachricht
        if tipp == geheime_zahl {
            let message = format!(
                "🎉 Richtig!\n\nDie Zahl war {}!\nDu hast {} Versuche gebraucht.",
                geheime_zahl, versuche
            );
            let _ = MessageBox {
                title: "Gewonnen!",
                message: &message,
                icon: MessageIcon::Info,
                buttons: MessageButtons::Ok,
                owner: None,
            }
            .show();
            break;
        } else if tipp < geheime_zahl {
            // ANCHOR_END: richtig_nachricht
            let _ = MessageBox {
                title: "Hinweis",
                message: "Zu klein! Versuche eine größere Zahl.",
                icon: MessageIcon::Info,
                buttons: MessageButtons::Ok,
                owner: None,
            }
            .show();
        } else {
            let _ = MessageBox {
                title: "Hinweis",
                message: "Zu groß! Versuche eine kleinere Zahl.",
                icon: MessageIcon::Info,
                buttons: MessageButtons::Ok,
                owner: None,
            }
            .show();
        }
    }

    println!("Spiel beendet!");
}
