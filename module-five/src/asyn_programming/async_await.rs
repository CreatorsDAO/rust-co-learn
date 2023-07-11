//! 异步实战
//!

/**

```
use reqwest::{blocking::Client, Error};
use serde::Deserialize;
use std::vec::Vec;
use tokio;

#[derive(Deserialize, Debug)]
struct Post {
    userId: u32,
    id: u32,
    title: String,
    body: String,
}

async fn fetch_post_async(url: &str) -> Result<Post, Error> {
    let response = reqwest::get(url).await?;
    let response = response.error_for_status()?;
    let post: Post = response.json().await?;
    Ok(post)
}

fn fetch_post_sync(url: &str) -> Result<Post, Error> {
    let client = Client::new();
    let response = client.get(url).send()?;
    let response = response.error_for_status()?;
    let post: Post = response.json()?;
    Ok(post)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let urls = vec![
        "https://jsonplaceholder.typicode.com/posts/1",
        "https://jsonplaceholder.typicode.com/posts/2",
        "https://jsonplaceholder.typicode.com/posts/3",
    ];

    let urls_clone = urls.clone();

    println!("同步请求：");

    let sync_start = std::time::Instant::now();
    let sync_thread = std::thread::spawn(move || {
        for url in &urls_clone {
            match fetch_post_sync(url) {
                Ok(post) => println!("Post信息: {:?}", post),
                Err(err) => eprintln!("请求失败: {:?}", err),
            }
        }
    });
    sync_thread.join().unwrap();
    let sync_duration = sync_start.elapsed().as_millis();
    println!("同步请求总耗时：{} ms", sync_duration);

    println!("异步请求：");
    let async_start = std::time::Instant::now();
    let mut tasks = Vec::new();
    for url in urls {
        let task = tokio::spawn(async move {
            match fetch_post_async(&url).await {
                Ok(post) => println!("Post信息: {:?}", post),
                Err(err) => eprintln!("请求失败: {:?}", err),
            }
        });
        tasks.push(task);
    }

    for task in tasks {
        task.await?;
    }
    let async_duration = async_start.elapsed().as_millis();
    println!("异步请求总耗时：{} ms", async_duration);

    let ratio = sync_duration as f64 / async_duration as f64;
    println!("同步请求耗时是异步请求耗时的 {:.2} 倍", ratio);

    Ok(())
}

```
*/

pub fn async_await() {
    println!("");
}

// Cargo.toml中的依赖项
// reqwest = { version = "0.11", features = ["blocking", "json"] }
// tokio = { version = "1", features = ["full"] }
// serde = { version = "1.0", features = ["derive"] }
