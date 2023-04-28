// use reqwest::{blocking::Client, Error};
// use serde::Deserialize;
// use std::vec::Vec;
// use tokio;

// #[derive(Deserialize, Debug)]
// struct Post {
//     userId: u32,
//     id: u32,
//     title: String,
//     body: String,
// }

// async fn fetch_post_async(url: &str) -> Result<Post, Error> {
//     let response = reqwest::get(url).await?;
//     let response = response.error_for_status()?;
//     let post: Post = response.json().await?;
//     Ok(post)
// }

// fn fetch_post_sync(url: &str) -> Result<Post, Error> {
//     let client = Client::new();
//     let response = client.get(url).send()?;
//     let response = response.error_for_status()?;
//     let post: Post = response.json()?;
//     Ok(post)
// }

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let urls = vec![
//         "https://jsonplaceholder.typicode.com/posts/1",
//         "https://jsonplaceholder.typicode.com/posts/2",
//         "https://jsonplaceholder.typicode.com/posts/3",
//     ];

//     let urls_clone = urls.clone();

//     println!("同步请求：");
//     let sync_thread = std::thread::spawn(move || {
//         for url in &urls_clone {
//             match fetch_post_sync(url) {
//                 Ok(post) => println!("Post信息: {:?}", post),
//                 Err(err) => eprintln!("请求失败: {:?}", err),
//             }
//         }
//     });
//     sync_thread.join().unwrap();

//     println!("异步请求：");
//     let mut tasks = Vec::new();
//     for url in urls {
//         let task = tokio::spawn(async move {
//             match fetch_post_async(&url).await {
//                 Ok(post) => println!("Post信息: {:?}", post),
//                 Err(err) => eprintln!("请求失败: {:?}", err),
//             }
//         });
//         tasks.push(task);
//     }

//     for task in tasks {
//         task.await?;
//     }

//     Ok(())
// }
fn main() {}
