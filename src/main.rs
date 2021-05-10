use std::time::{Duration, Instant};

const TASKS: usize = 100;

#[tokio::main]
async fn main() -> reqwest::Result<()> {
    let client = reqwest::Client::builder()
        .connect_timeout(Duration::from_secs(2))
        .build()
        .unwrap();

    let responses = futures::future::join_all((0..TASKS).map(|n| request(n, client.clone()))).await;

    for (i, res) in responses.iter().enumerate() {
        if let Ok((resp, dur)) = res {
            println!("Result #{} ({}µs): {:?}", i, dur.as_micros(), resp);
        }
    }

    let sum = responses.iter().fold(Duration::default(), |d1, r| {
        if let Ok((_, d2)) = r {
            d1 + *d2
        } else {
            d1
        }
    });
    println!("Sum: {}ms", sum.as_millis());
    println!("Avg: {}ms", (sum / TASKS as u32).as_millis());

    Ok(())
}

async fn request(n: usize, client: reqwest::Client) -> reqwest::Result<(String, Duration)> {
    println!("This is request #{}", n);
    let start = Instant::now();
    let response = client
        .get(format!(
            "https://jsonplaceholder.typicode.com/todos/{}",
            n + 1
        ))
        .send()
        .await?
        .text()
        .await?;
    Ok((response, Instant::now().duration_since(start)))
}
