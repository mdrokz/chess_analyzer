use headless_chrome::{Browser, LaunchOptions};

const GET_MATCH_URLS_SCRIPT: &'static str = include_str!("get_match_urls.js");

const GET_PAGES_SCRIPT: &'static str = include_str!("get_pages.js");

fn main() {
    let browser = Browser::new(
        LaunchOptions::default_builder()
            .headless(false)
            .build()
            .unwrap(),
    )
    .unwrap();

    let tab = browser.wait_for_initial_tab().unwrap();

    tab.navigate_to("https://www.chess.com/login").unwrap();
    // tab.navigate_to("https://www.chess.com/games/archive/kusaik?gameOwner=other_game&gameTypes%5B0%5D=chess960&gameTypes%5B1%5D=daily&gameType=live").unwrap();

    tab.wait_until_navigated().unwrap();

    let (username, password, login) = (
        tab.find_element("#username").unwrap(),
        tab.find_element("#password").unwrap(),
        tab.find_element("#login").unwrap(),
    );

    username.type_into("mdrokz3").unwrap();
    password.type_into("maramune18").unwrap();
    login.click().unwrap();

    std::thread::sleep(std::time::Duration::from_secs(3));

    tab.navigate_to("https://www.chess.com/games/archive/kusaik?gameOwner=other_game&gameTypes%5B0%5D=chess960&gameTypes%5B1%5D=daily&gameType=live").unwrap();

    tab.wait_until_navigated().unwrap();

    let pages = tab
        .evaluate(GET_PAGES_SCRIPT, false)
        .unwrap()
        .value
        .unwrap()
        .as_i64()
        .unwrap();

    println!("{:?}", pages);

    browser.new_tab().unwrap();

    std::thread::sleep(std::time::Duration::from_secs(1000));
    println!("Hello, world!");
}
