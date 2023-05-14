
#[tokio::main]
async fn main() {
    let url = "https://www.youtube.com/watch?v=HMKUlsJpov8";
    println!(
        "downloaded video to {:?}",
        rustube::download_best_quality(&url).await.unwrap()
    );
}
