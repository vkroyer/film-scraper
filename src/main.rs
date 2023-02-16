use scraper::{Html, Selector};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    find_upcoming().await
}

async fn find_upcoming() -> Result<(), Box<dyn std::error::Error>> {

    let res = reqwest::get("INSERT ACTOR IMDB URL")
        .await?;
    
    let content = res.text().await.unwrap();

    let document = Html::parse_document(&content);

    let selector = Selector::parse("li[class*=unreleased]").unwrap();

    let elements = document.select(&selector);

    let href_selector = Selector::parse("a").unwrap();

    // Loop through the elements and print their text content
    for element in elements {
        let title_element = element.select(&href_selector).next().unwrap();
        let title = title_element.text().collect::<String>();
        let url = title_element.value().attr("href").unwrap_or("");
        let url = format!("https://imdb.com{}", url);
        println!("{}\n(url: {})\n\n", title, url)
    }


    Ok(())
}
