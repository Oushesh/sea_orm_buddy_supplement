# sea_orm_buddy_supplement
Test for a model before I switch from sqlitex to seaORM. Checking and reading the pros and cons

## Migrations: 

## Rust Based Crawler
   Inspired from: Firecrawl
   
## Self made Crawl
https://github.com/firecrawl/firecrawl/tree/main/apps/api/src/scraper/scrapeURL

## TBD: To be determined how to fetch the different types of data

## ScrapeURL Logic Flow:
    
    scrapeURL/index.ts --> https://github.com/firecrawl/firecrawl/tree/main/apps/api/src/scraper/scrapeURL

    1. 


## Crawler Structure for implementing a rust version of Firecrawl: 
   

## Derived Software Architecture: 
   1. The Strategy Pattern (Rust Enums)
      In the firecrawl, they decide whether to use a simple fetch or a full browser.
      In rust, you would model this with an enum: 
      
   """
    enum ScrapeStrategy {
        Fast, // Using 'reqwest' (Raw HTML, no JS)
        Browser. // Using 'headless_chrome' (JS Rendering)
    }   
   """
      
    2. The Step-by-Step Porting Guide
    Step A: The Fetcher (Networking)

    Step B: The Content Discovery (DOM Traversal)
    * Firecrawl logic: Found in apps/
    * Rust Implementation: Use the scraper crate. 
      You can write a "Readiness" function that checks
      for the existence of <p> tags or a specific div.

    Step C: The Markdown Engine
    * Firecrawl logic: Uses turndown.
    * Rust Implementation: The html2md crate is the closest equivalent,
      but for a "Firecrawl-quality" output, you should
      pipe your HTML through the ammonia crate first to sanitize
      and remove the "noise" tags (<nav>, <script>, etc..)
      discovered by the algorithm we discussed.

    Why this is a good starting point in Rust? 
    
    The scrapeURL folder is mostly "pure" logic -- it takes 
    a URL and returns a string. 

    You dont have to worry about the Redis Queues (BullMQ) or the 
    complex API routing yet. (BullMQ)

    One possible strategy is to let's say use: 
    3 different .rs files: 

    1. src/fetcher.rs --> handles "reqwest" Chrome vs headless_Chrome
    2. src/cleaner.rs --> Uses ammonia to strip junk
    3. src/converter.s --> after cleaning the junk we convert the cleaned/processed html tags to md file

## Folder Structure:
   We have 3 Apps in the main folder structure.
   App 1: Supplement
   App 2: user
   App 3: under folder source

    sea_orm_buddy_supplement/  <-- Root Folder
    ├── Cargo.toml             <-- Workspace Definition
    ├── supplement/            <-- Your App
    │   ├── Cargo.toml
    │   └── src/
    │       └── main.rs
    └── user/                  <-- Your other App
    ├── Cargo.toml
    └── src/
    └── main.rs


## Tip afterwards:
   If you look at the Firecrawl source, they often use "Base64 Image Handling",
   When scraping for lLMs, you usually want to strip images or convert them to 
   just their alt text to save tokens. Implementing a toggle in code 
   to strip_images(true) will immediately make your tool
   more "AI-native" more than 90% of other scrapers.

   
   
   

    