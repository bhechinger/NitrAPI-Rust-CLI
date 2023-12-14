#[tokio::main]
async fn main() {
    let result = nitrapi::ping::all().await;

    if let Ok(ping) = result{
        println!("{}", ping)
    };
}
