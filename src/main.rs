use headless_chrome::{Browser, LaunchOptions};

fn main() {
    let browser = Browser::new(
        LaunchOptions::default_builder()
            .headless(false)
            .build()
            .unwrap(),
    )
    .unwrap();


    let tab = browser.wait_for_initial_tab().unwrap();

    

    println!("Hello, world!");
}
