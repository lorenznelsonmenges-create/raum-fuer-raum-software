use bcrypt::verify;

#[test]
fn test_reproduce_bcrypt_error() {
    let password = "achtsam2024";
    // Der alte Hash aus 20240410000000_add_users.sql war ungültig (zu kurz)
    let bad_hash = "$2b$12$6P29FhZ/9.uX098e9XqgHe6GkE1/yP5vU8/fO1L8f.7G.z7.0.8";
    assert!(verify(password, bad_hash).is_err(), "Der fehlerhafte Hash muss einen Fehler werfen");

    // Der korrigierte Hash aus 20240411000000_fix_admin_hash.sql muss funktionieren
    let fixed_hash = "$2b$12$s4nMxf4upx/0DKHOmhaU7OPAV5COxzxrFzZrrpMrtSuoQjVquHhDC";
    assert!(verify(password, fixed_hash).unwrap(), "Der korrigierte Hash muss erfolgreich verifizieren");
}
