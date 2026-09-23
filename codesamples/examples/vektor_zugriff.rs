//! Beispiel: Auf Elemente zugreifen
//!
//! Dieses Programm zeigt, wie man auf einzelne Elemente zugreift.

fn main() {
    let fruechte = vec!["Apfel", "Banane", "Kirsche", "Dattel"];
    
    println!("Erstes Element: {}", fruechte[0]);
    println!("Zweites Element: {}", fruechte[1]);
    println!("Letztes Element: {}", fruechte[3]);
    
    // Index in einer Variable
    let index = 2;
    println!("Element an Position {}: {}", index, fruechte[index]);
}
