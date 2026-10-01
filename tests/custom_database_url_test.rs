use std::env;
use std::fs;
use wendepunkt_software::database;

#[tokio::test]
async fn test_init_db_custom_url() {
    let test_db = "custom_test_env.db";
    if std::path::Path::new(test_db).exists() {
        let _ = fs::remove_file(test_db);
    }
    env::set_var("DATABASE_URL", format!("sqlite:{}", test_db));
    let pool = database::init_db().await.expect("Failed to init db with custom url");
    
    assert!(std::path::Path::new(test_db).exists(), "Custom db file should be created");

    drop(pool);
    env::remove_var("DATABASE_URL");
    let _ = fs::remove_file(test_db);
}
