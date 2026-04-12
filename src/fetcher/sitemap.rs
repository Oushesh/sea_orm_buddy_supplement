//Tip: make sure the functions under are marked
//as public other main cant see them.

/*
pub fn start_fetch(url:&str) {
    println!("Fetching {}...",url);
}
*/

use headless_chrome::{Browser,LaunchOptions};
use std::time::Duration;

pub async fn fetch_html(url: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) Chrome/122.0.0.0")
        .build()?;

    let response = client.get(url).send().await?;

    //Print the status so you know if you got blocked (403) or found
    println!("Status: {}", response.status());
    response.text().await
}

//This is the more advanced version of the code
//where we try to fetch with the browser for the demo comes:
pub async fn fetch_with_browser(url:&str) ->Result<String, Box<dyn std::error::Error>> {
    //1. Launch Options - Optimized for 8GB RAM
    let options = LaunchOptions::default_builder()
        .headless(true)
        .build()?;

    let browser = Browser::new(options)?;
    let tab = browser.new_tab()?;

    // 2. Navigate
    tab.navigate_to(url)?;

    // 3. Stealth: Hide the "Automated" flag
    tab.evaluate("Object.defineProperty(navigator, 'webdriver', {get: () => undefined})", false)?;

    //4. Wait for the "Human Challenge" (PerimeterX/Cloudflare)
    // On an M2, this is where the CPU does the heavy lifting
    tokio::time::sleep(Duration::from_secs(7)).await;

    //5. Grab the rendered HTML
    let html = tab.get_content()?;
    Ok(html)

}

pub async fn fetch_stealth(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let options = LaunchOptions::default_builder().headless(true).build()?;
    let browser = Browser::new(options)?;
    let tab = browser.new_tab()?;

    // CRITICAL: This script overwrites the 'automation' signatures
    // that PerimeterX looks for immediately.
    tab.evaluate(r#"
        (() => {
            // Delete the webdriver property
            delete Object.getPrototypeOf(navigator).webdriver;
            
            // Mock Chrome specific properties
            window.chrome = { runtime: {} };
            
            // Mock permissions
            const originalQuery = window.navigator.permissions.query;
            window.navigator.permissions.query = (parameters) => (
                parameters.name === 'notifications' ?
                Promise.resolve({ state: Notification.permission }) :
                originalQuery(parameters)
            );
        })()
    "#, false)?;

    tab.navigate_to(url)?;

    // Firecrawl waits a random amount of time to simulate human 'thinking'
    tokio::time::sleep(Duration::from_secs(5)).await;

    Ok(tab.get_content()?)
}