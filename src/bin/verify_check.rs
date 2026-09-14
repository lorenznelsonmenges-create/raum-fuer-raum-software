use bcrypt::verify;

fn main() {
    let password = "achtsam2024";
    let hash = "$2b$12$s4nMxf4upx/0DKHOmhaU7OPAV5COxzxrFzZrrpMrtSuoQjVquHhDC";
    
    match verify(password, hash) {
        Ok(true) => println!("VERIFIED: TRUE"),
        Ok(false) => println!("VERIFIED: FALSE"),
        Err(e) => println!("ERROR: {:?}", e),
    }
}
