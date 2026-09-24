use bcrypt::{hash, DEFAULT_COST};

fn main() {
    let password = "stefanie2026!";
    let hashed = hash(password, DEFAULT_COST).unwrap();
    println!("{}", hashed);
}
