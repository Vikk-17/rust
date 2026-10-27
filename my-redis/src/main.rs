use trpl::Html;

// async block
async fn page_title(url: &str) -> Option<String> {
    let response_text = trpl::get(url).await.text().await;
    Html::parse(&response_text)
        .select_first("title")
        .map(|title| title.inner_html())
}

// Rust does this internally
// fn page_title(url: &str) -> impl Future<Output = Option<String>> {
//     async move {
//         let response_text = trpl::get(url).await.text().await;
//         Html::parse(&response_text)
//             .select_first("title")
//             .map(|title| title.inner_html())
//     }
// }

fn main() {
    let args: Vec<String> = std::env::args().collect();

    trpl::block_on(async {
        let url = &args[1];
        match page_title(url).await {
            Some(title) => println!("The title for {url} was {title}"),
            None => println!("{url} had no title"),
        }
    })
}
