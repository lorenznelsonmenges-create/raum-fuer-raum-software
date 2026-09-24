use bcrypt::verify;
use sqlx::sqlite::SqlitePoolOptions;
use wendepunkt_software::database;

#[tokio::test]
async fn test_db_auth_admin_and_stefanie() {
    let pool = SqlitePoolOptions::new()
        .connect("sqlite::memory:")
        .await
        .expect("Konnte In-Memory-DB nicht öffnen");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Migrationen fehlgeschlagen");

    // 1. Admin prüfen
    let admin_user = database::get_user_by_username(&pool, "admin")
        .await
        .expect("Admin-Benutzer nicht in DB gefunden");
    assert_eq!(admin_user.role, "ADMIN");
    assert!(
        verify("achtsam2024", &admin_user.password_hash).unwrap_or(false),
        "Admin-Passwort 'achtsam2024' konnte nicht verifiziert werden!"
    );

    // 2. Stefanie prüfen
    let stefanie_user = database::get_user_by_username(&pool, "stefanie")
        .await
        .expect("Stefanie-Benutzer nicht in DB gefunden");
    assert_eq!(stefanie_user.role, "ADMIN");
    assert!(
        verify("stefanie2026!", &stefanie_user.password_hash).unwrap_or(false),
        "Stefanie-Passwort 'stefanie2026!' konnte nicht verifiziert werden!"
    );

    // 3. Falsches Passwort abweisen
    assert!(
        !verify("falsches_passwort", &stefanie_user.password_hash).unwrap_or(true),
        "Falsches Passwort sollte abgelehnt werden!"
    );
}
