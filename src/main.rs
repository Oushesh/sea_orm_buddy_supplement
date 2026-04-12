//This function will call the crawler from src/crawler_buddy/* scripts here
//module declaration similar to import in python

mod fetcher;
mod crawler;
mod converter;

use fetcher::sitemap::fetch_html;
use crawler::sitemap::parse_sitemap;
use converter::sitemap::converter;

#[tokio::main]
async fn main () {
    //1. Cal the function using the module
    let url = "https://www.olvlimits.com";
    fetch_html(url).await.unwrap();

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
}

//Architecture and Design in Rust.

//Finish building this crawler in Rust.

//TODO: <architecture of files import>


//TODO:
