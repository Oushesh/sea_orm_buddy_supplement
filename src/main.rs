//This function will call the crawler from src/crawler_buddy/* scripts here
//module declaration similar to import in python

mod fetcher;
mod crawler;
mod converter;

use fetcher::sitemap::start_fetch;
use crawler::sitemap::parse_sitemap;
use converter::sitemap::converter;

fn main () {
    //1. Cal the function using the module
    start_fetch("https://www.olvlimits.com");

    //2. Call the function using the folder module
    parse_sitemap();

    //3. Call the function using
    converter();
}

//Architecture and Design in Rust.

//Finish building this crawler in Rust.

//TODO: <architecture of files import>
