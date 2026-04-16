//This function will call the crawler from src/crawler_buddy/* scripts here
//module declaration similar to import in python

mod fetcher;
mod crawler;
mod converter;

use fetcher::sitemap::{fetch_html,fetch_with_browser,fetch_stealth};
use crawler::sitemap::parse_sitemap;
use converter::sitemap::converter;
use oxymouse_rs::algorithms::bezier_old::BezierMouse;

#[tokio::main]
async fn main () {
    //1. Cal the function using the module
    //let url = "https://www.olvlimits.com";
    //let url = "https://news.ycombinator.com";
    let url = "https://www.zillow.com/";
    //fetch_html(url).await.unwrap();
    fetch_stealth(url).await.unwrap();
    println!("📡 Fetching HTML from {}...", url);


    // Now .await is allowed!
    match fetch_html(url).await {
        Ok(html) => {
            println!("✅ Success! Total length: {} characters", html.len());

        //Let's see the first 200 chars to sure
        let preview = &html[0..200.min(html.len())];
        println!("--- Preview ---\n{}\n---------------", preview);
        }

        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
    //2. Call the function using the folder module
    parse_sitemap();

    //3. Call the function using
    converter();

    //4. Demo: generate Bézier mouse-movement coordinates via oxymouse_rs
    let movements = BezierMouse::generate_coordinates(0, 0, 1920, 1080);
    if let (Some(first), Some(last)) = (movements.first(), movements.last()) {
        println!(
            "🖱️  BezierMouse: {} steps from (0,0) to (1920,1080). First={:?}, Last={:?}",
            movements.len(),
            first,
            last,
        );
    }
    
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

Token Efficiency: Because Markdown is much shorter than HTML,
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

/*
TODO: <Add also a graph to export the images.>
 */