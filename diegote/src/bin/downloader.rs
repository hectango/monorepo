#[tokio::main]
async fn main() {
    let url = "https://www.youtube.com/watch?v=MepPfI7ebMY";
    println!(
        "downloaded video to {:?}",
        rustube::download_best_quality(&url).await.unwrap()
    );
}
