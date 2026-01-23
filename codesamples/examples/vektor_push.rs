//! Beispiel: Elemente hinzufügen
//!
//! Dieses Programm zeigt, wie man Elemente zu einem Vektor hinzufügt.

fn main() {
    let mut zahlen = Vec::new();
    
    println!("Leerer Vektor: {:?}", zahlen);
    
    zahlen.push(5);
    zahlen.push(10);
    zahlen.push(15);
    
    println!("Nach push: {:?}", zahlen);
    
    zahlen.push(20);
    println!("Noch mehr: {:?}", zahlen);
}
