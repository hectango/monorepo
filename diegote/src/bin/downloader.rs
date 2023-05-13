#[tokio::main]
async fn main() {
    let url = "https://www.youtube.com/watch?v=Edx9D2yaOGs&ab_channel=CollegeHumor";
    println!(
        "downloaded video to {:?}",
        rustube::download_best_quality(&url).await.unwrap()
    );
}
