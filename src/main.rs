//This function will call the crawler from src/crawler_buddy/* scripts here
//module declaration similar to import in python

mod fetcher;
mod crawler;
mod converter;

use fetcher::sitemap::{fetch_html,fetch_with_browser,fetch_stealth_simplified};
use crawler::sitemap::parse_sitemap;
use converter::sitemap::to_markdown;

use rand::seq::SliceRandom;
use rand::Rng;

// Add rand to Cargo.toml

#[tokio::main]
async fn main () {
    //1. Cal the function using the module
    //let url = "https://www.olvlimits.com";
    //let url = "https://news.ycombinator.com";
    let url = "https://www.zillow.com/";
    //fetch_html(url).await.unwrap();

    match fetch_stealth_simplified(url).await {
        Ok(html) => {
            println!("✅ HTML Retrieved! Length: {} characters", html.len());

            // 2. Pass the HTML to your Refinery
            let markdown = to_markdown(&html);

            println!("--- REFINED MARKDOWN PREVIEW ---");
            // Print the first 500 characters so we don't flood the terminal
            println!("{}", &markdown[..std::cmp::min(markdown.len(), 500)]);
            println!("\n--- END PREVIEW ---");
        },
        Err(e) => {
            eprintln!("❌ The Stealth Mission Failed: {}", e);
        }
    }
    // A pool of potential proxy gateways
    /*
    let mut proxy_pool = vec![
        "http://proxy-us.example.com:8001",
        "http://proxy-eu.example.com:8001",
        "http://proxy-mobile.example.com:8001",
    ];

    let max_retries = 3;
    let mut attempt = 0;
    let mut success = false;


    while attempt < max_retries && !success {
        // 1. Pick a proxy
        let current_proxy = proxy_pool.choose(&mut rand::thread_rng()).unwrap();

        println!("🔄 Attempt {}/{} using proxy: {}", attempt + 1, max_retries, current_proxy);

        // 2. Try the fetch (Pass the proxy to your fetcher)
        match fetch_with_proxy(url, current_proxy).await {
            Ok(html) => {
                // Check if it's the REAL content or a Captcha
                if html.contains("px-captcha") {
                    println!("⚠️ Hit a Captcha! Rotating proxy...");
                } else {
                    let markdown = to_markdown(&html);
                    println!("✅ Success! Data extracted.");
                    success = true;
                    // Save or process your markdown here
                }
            }
            Err(e) => {
                eprintln!("❌ Network Error: {}. Trying next proxy...", e);
            }
        }

        attempt += 1;

        // Exponential backoff: Wait a bit longer each time to look "human"
        if !success {
            let wait_time = std::time::Duration::from_secs(attempt * 2);
            tokio::time::sleep(wait_time).await;
        }
    }

    if !success {
        println!("🛑 All retries failed. The site is winning today.");
    }
    */



    //2. Call the function using the folder module
    parse_sitemap();

    //3. TODO: option is call the converter here.
}

//Architecture and Design in Rust.

//Finish building this crawler in Rust.

//TODO: <architecture of files import>

//TODO: Economics of this Idea:
//Competitor Analysis:
//<TheLLM-Ready Output>

/*
"""
The LLM-Ready Output (The Main Advantage)
Markdown vs. HTML: Most scrapers return raw HTML, which
is bloated with <scripts> and <div> tags.

Token Efficiency: Because Markdownw is much shorter than HTML,
it uses roughly 60-70 % fewer tokens, making it significantly chaper
and faster to feed into models like GPT-4o or Claude.

Markdown preserves the sturture:
It maintains headings, lists and tables which are crucial
for an LLM to understand the hierarchy of information (key for
RAG systems).

2. Preserved Structure: It maintains headings, lists and tables, which are crucial
for an LLM to understand the hierarchy of information
(key for RAG systems).

2. Infrastructure as a Service
In the past, to scrape at scale, you had to manage:

Headless Browsers: Managing Playwright or Puppeteer instances (CPU/RAM heavy).

Proxy Rotation: Buying and rotating residential IPs to avoid being blocked.

Bot Detection: Handling CAPTCHAs and "stealth" headers.
Firecrawl handles all of this natively. You send a URL to their API, and they return the data. It's "serverless" scraping.

"""


"""
3. 3. Purpose-Built for AI AgentsFirecrawl includes features specifically designed for the AI Agent \
era that older tools lack:The /map Endpoint: Instantly discover all URLs on a website without having to manually parse a sitemap. This allows an agent to "know" the structure of a site in seconds.The /scrape Actions: You can tell Firecrawl to "click the button," "scroll down," or "wait for X to appear" before extracting, which is essential for modern Javascript-heavy apps (React/Next.js).Natural Language Navigation: Their newer "Agent" features allow you to say, "Find the pricing page and extract the pro tier features," rather than providing a specific URL or CSS selector.
"""
*/