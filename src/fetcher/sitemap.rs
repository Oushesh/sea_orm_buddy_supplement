//Tip: make sure the functions under are marked
//as public other main cant see them.

/*
pub fn start_fetch(url:&str) {
    println!("Fetching {}...",url);
}
*/


pub async fn fetch_html(url: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) Chrome/122.0.0.0")
        .build()?;

    let response = client.get(url).send().await?;

    //Print the status so you know if you got blocked (403) or found
    println!("Status: {}", response.status());
    response.text().await
}