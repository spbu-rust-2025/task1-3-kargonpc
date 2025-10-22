use std::fs;

fn main() {
    let mut p = String::new();
    std::io::stdin().read_line(&mut p).unwrap();
    
    if fs::read(p.trim()).is_ok() {
        println!("success");
    } else {
        println!("failure");
    }
}