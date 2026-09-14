use bcrypt::{hash, DEFAULT_COST};

fn main() {
    let password = "achtsam2024";
    let hashed = hash(password, DEFAULT_COST).unwrap();
    println!("{}", hashed);
}
