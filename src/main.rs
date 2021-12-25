use headless_chrome::{Browser, LaunchOptions};

use std::time::Duration;

use std::thread;

const GET_MATCH_URLS_SCRIPT: &'static str = include_str!("get_match_urls.js");

const GET_PAGES_SCRIPT: &'static str = include_str!("get_pages.js");

macro_rules! element {
    ($t: expr,$($v: literal),*) => {
        ($(
            $t.find_element($v).expect(&format!("element for this selector {} was not found",$v)),
        )*)
    };

    ($t: expr,$v: literal) => {
        $t.find_element($v).expect(&format!("element for this selector {} was not found",$v))
    };
}

macro_rules! collect_strings {
    ($o: expr) => {
        $o.preview
            .unwrap()
            .properties
            .iter()
            .map(|f| f.value.clone().unwrap())
            .collect::<Vec<String>>()
    };
}

fn main() {
    let mut args = std::env::args();

    args.next();

    // get target profile name from program arguments
    let profile_name = args.next().expect("profile_name is empty");

    // get username for login from program arguments
    let username = args.next().expect("username is empty");

    // get password for login from program arguments
    let password = args.next().expect("password is empty");

    // format url with the profile name
    let url = format!("https://www.chess.com/games/archive/{}?gameOwner=other_game&gameTypes%5B0%5D=chess960&gameTypes%5B1%5D=daily&gameType=live",profile_name);

    let browser = Browser::new(
        LaunchOptions::default_builder()
            .headless(false)
            .idle_browser_timeout(Duration::from_secs(10000 * 60))
            .build()
            .unwrap(),
    )
    .unwrap();

    let tab = browser.wait_for_initial_tab().unwrap();

    tab.navigate_to("https://www.chess.com/login").unwrap();

    tab.wait_until_navigated().unwrap();

    // fetch username,password and button elements
    let (u, p, l) = element!(tab, "#username", "#password", "#login");

    /* type username and password into input and click on login button */
    u.type_into(&username).unwrap();
    p.type_into(&password).unwrap();
    l.click().unwrap();

    // wait for login
    thread::sleep(Duration::from_secs(3));

    tab.navigate_to(&url).unwrap();

    tab.wait_until_navigated().unwrap();

    // get total pages for pagination
    let pages = tab
        .evaluate(GET_PAGES_SCRIPT, false)
        .unwrap()
        .value
        .unwrap()
        .as_i64()
        .unwrap();

    /* open all match urls for each page to trigger analysis */
    for i in 1..pages + 2 {
        if i >= 2 {
            let mut new_url = url.clone();

            new_url.push_str(&format!("&page={}", i));

            tab.navigate_to(&new_url).unwrap();

            tab.wait_until_navigated().expect("failed to navigate");
        }

        /*  get all match urls from the table */
        let urls_object = tab.evaluate(GET_MATCH_URLS_SCRIPT, false).unwrap();

        let match_urls = collect_strings!(urls_object);

        let mut tabs = vec![];

        for url in match_urls {
            let new_tab = browser.new_tab().unwrap();

            new_tab.navigate_to(&url).unwrap();

            new_tab.wait_until_navigated().expect("failed to navigate");

            tabs.push(new_tab);
        }

        for t in tabs {
            t.close(false).unwrap();
        }
    }

    println!("Completed!");
}
