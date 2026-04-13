use bcrypt::verify;

#[test]
fn test_reproduce_bcrypt_error() {
    let password = "achtsam2024";
    // Der Hash aus migrations/20240410000000_add_users.sql
    let bad_hash = "$2b$12$6P29FhZ/9.uX098e9XqgHe6GkE1/yP5vU8/fO1L8f.7G.z7.0.8";
    
    println!("Versuche Passwort '{}' gegen Hash '{}' zu verifizieren", password, bad_hash);
    
    let result = verify(password, bad_hash);
    
    match result {
        Ok(is_valid) => {
            println!("Verifikation erfolgreich (Ergebnis: {})", is_valid);
            assert!(is_valid, "Passwort sollte gültig sein, wenn der Hash korrekt wäre");
        }
        Err(e) => {
            println!("Reproduzierter Fehler: {:?}", e);
            // Wir erwarten hier einen Fehler, weil der Hash ungültig ist (zu kurz).
            panic!("Bcrypt-Verifikationsfehler aufgetreten: {:?}", e);
        }
    }
}
