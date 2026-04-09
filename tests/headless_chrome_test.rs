use headless_chrome::{Browser, LaunchOptions};
use std::ffi::OsStr;

#[test]
fn test_headless_chrome_initialization() {
    let options = LaunchOptions::default_builder()
        .headless(true)
        .args(vec![
            OsStr::new("--no-sandbox"),
            OsStr::new("--disable-setuid-sandbox"),
            OsStr::new("--disable-dev-shm-usage"),
        ])
        .build()
        .expect("Failed to build LaunchOptions");

    println!("Attempting to launch browser...");
    let browser = Browser::new(options);
    
    match browser {
        Ok(_browser) => {
            println!("Successfully initialized headless_chrome browser!");
            let tab = _browser.new_tab().expect("Failed to open a new tab");
            println!("Successfully opened a new tab!");
            
            // Navigate to a simple data URL
            tab.navigate_to("data:text/html;base64,PGgxPkhlbGxvIFdvcmxkPC9oMT4=") // <h1>Hello World</h1>
                .expect("Failed to navigate to data URL");
            tab.wait_until_navigated().expect("Failed to wait for navigation");
            
            let pdf_data = tab.print_to_pdf(None).expect("Failed to print to PDF");
            assert!(!pdf_data.is_empty(), "PDF data should not be empty");
            println!("Successfully generated a PDF of {} bytes!", pdf_data.len());
        },
        Err(e) => {
            panic!("Failed to initialize headless_chrome browser: {}. Is Chromium/Chrome installed and accessible?", e);
        }
    }
}
