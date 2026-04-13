//Tip: make sure the functions under are marked
//as public other main cant see them.

/*
pub fn start_fetch(url:&str) {
    println!("Fetching {}...",url);
}
*/

use reqwest::{Proxy, Client};
use headless_chrome::{Browser, LaunchOptions};
use std::time::Duration;
use rand::Rng;

// FIX 1: Import the exact types the library version 1.0.21 expects
use headless_chrome::protocol::cdp::Input::MouseButton;

use headless_chrome::protocol::cdp::types;


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

/*

pub async fn fetch_stealth(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let options = LaunchOptions::default_builder()
        .headless(true)
        .window_size(Some((1920, 1080)))
        .build()?;

    let browser = Browser::new(options)?;
    let tab = browser.new_tab()?;

    // 1. JS Evasion
    tab.evaluate(r#"
        (() => {
            delete Object.getPrototypeOf(navigator).webdriver;
            window.chrome = { runtime: {} };
        })()
    "#, false)?;

    tab.navigate_to(url)?;

    // 2. The Interaction Layer
    if let Ok(element) = tab.wait_for_element_with_custom_timeout("div[id*='px-captcha']", Duration::from_secs(5)) {
        println!("⚠️ Captcha detected! Initiating 'Press & Hold' bypass...");

        let box_model = element.get_box_model()?;

        // FIX 2: Use top_left (which 1.0.21 uses in ElementQuad)
        let x = box_model.content.top_left.x + (box_model.width as f64 / 2.0);
        let y = box_model.content.top_left.y + (box_model.height as f64 / 2.0);

        // FIX 3: Pass the specific 'Point' struct the function requires
        tab.move_mouse_to_point(Point { x, y })?;

        // FIX 4: Use the MouseButton enum correctly
        tab.press_mouse_button(MouseButton::Left, 1)?;

        let hold_duration = rand::thread_rng().gen_range(3500..5000);
        tokio::time::sleep(Duration::from_millis(hold_duration)).await;

        tab.release_mouse_button(MouseButton::Left, 1)?;

        println!("✅ Hold completed.");
        tokio::time::sleep(Duration::from_secs(10)).await;
    }

    tokio::time::sleep(Duration::from_secs(5)).await;
    Ok(tab.get_content()?)
}
// --- OPTION A: THE FAST PATH (HTTP CLIENT) ---
pub async fn fetch_with_proxy(url: &str, proxy_url: &str) -> Result<String, reqwest::Error> {
    // 1. Configure the Proxy
    let proxy = Proxy::all(proxy_url)?;

    // 2. Build the Stealth Client
    let client = Client::builder()
        .proxy(proxy)
        .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36")
        .timeout(Duration::from_secs(15))
        .danger_accept_invalid_certs(true) // Useful if testing with Charles/Proxyman
        .build()?;

    // 3. Execute
    let response = client.get(url).send().await?;

    // Status check
    println!("📡 Proxy Status: {}", response.status());

    response.text().await
}

*/
pub async fn fetch_stealth_simplified(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let options = LaunchOptions::default_builder().headless(true).build()?;
    let browser = Browser::new(options)?;
    let tab = browser.new_tab()?;

    tab.navigate_to(url)?;

    // Spider-rs often uses "Wait for Selector" rather than manual timers.
    // This is much more stable.
    let _ = tab.wait_for_element("body")?;

    // If a captcha exists, Spider-rs would use a specialized "Action"
    // We can simulate that with a single high-level command:
    if let Ok(captcha) = tab.wait_for_element_with_custom_timeout("div[id*='px-captcha']", Duration::from_secs(5)) {
        println!("🤖 Automated Action: Bypassing Security...");

        // Instead of manual X/Y math, just use the built-in 'click'
        // which most modern versions of the crate have fixed:
        captcha.click()?;
    }

    Ok(tab.get_content()?)
}