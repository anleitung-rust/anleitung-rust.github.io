//! Beispiel: Text ausgeben
//!
//! Dieses Programm zeigt, wie man Text ausgibt.

fn main() {
    let gruss = String::from("Hallo");
    let name = String::from("Welt");
    
    println!("{}", gruss);
    println!("{}", name);
    println!("{} {}!", gruss, name);
}
