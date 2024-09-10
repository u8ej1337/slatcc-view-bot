use reqwest::{Client, Proxy, StatusCode};
use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead},
    path::Path,
    sync::{atomic::{AtomicU32, Ordering}, Arc}
};
use tokio;
use winconsole;
use rand::seq::SliceRandom;
use ansi_term::Colour::*;

// stats
struct Stats {
    views_sent: AtomicU32,
    views_failed: AtomicU32,
}

// build proxy url
fn build_proxy(url: &str) -> Option<Proxy> {
    Proxy::all(&format!("http://{}", url)).ok()
}

// read proxies
fn read_proxies_from_file(file_path: &str) -> io::Result<Arc<Vec<String>>> {
    let path: &Path = Path::new(file_path);
    let file: File = File::open(&path)?;
    let reader: io::BufReader<File> = io::BufReader::new(file);
    let proxies: HashSet<String> = reader.lines()
        .filter_map(|line: Result<String, io::Error>| line.ok())
        .map(|line: String| line.trim().to_string())
        .collect();

    Ok(Arc::new(proxies.into_iter().collect()))
}

// send request
async fn send_slat_request(user_id: u32, proxies: Arc<Vec<String>>, stats: Arc<Stats>) {
    let proxy_url: &String = match proxies.choose(&mut rand::thread_rng()) {
        Some(url) => url,
        None => return,
    };
    let proxy: Proxy = match build_proxy(proxy_url) {
        Some(proxy) => proxy,
        None => return,
    };

    let client_with_proxy: Client = Client::builder()
        .proxy(proxy)
        .build()
        .unwrap();

    let response: Result<reqwest::Response, reqwest::Error> = client_with_proxy
        .post(&format!("https://slat.cc/api/users/{}/views", user_id))
        .header("accept", "*/*")
        .header("accept-language", "en-US,en;q=0.5")
        .header("content-type", "application/json")
        .header("priority", "u=1, i")
        .header("sec-ch-ua-mobile", "?0")
        .header("sec-ch-ua-platform", "\"Windows\"")
        .header("sec-fetch-dest", "empty")
        .header("sec-fetch-mode", "cors")
        .header("sec-fetch-site", "same-origin")
        .header("sec-gpc", "1")
        .header("User-Agent", reqwest::header::HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/100.0.4896.60 Safari/537.36"))
        .header("Referer", "https://slat.cc/")
        .send()
        .await;

    match response {
        Ok(res) => {
            if res.status() == StatusCode::OK {
                stats.views_sent.fetch_add(1, Ordering::SeqCst);
                println!("{}", Green.paint("Successfully sent view to the slat.cc profile you requested!"));
            } else {
                stats.views_failed.fetch_add(1, Ordering::SeqCst);
                println!("{}", Red.paint("Failed to send view..."));
            }
        },
        Err(_) => {}
    }
}

// update title
async fn update_title(stats: Arc<Stats>) {
    loop {
        let sent: u32 = stats.views_sent.load(Ordering::SeqCst);
        let failed: u32 = stats.views_failed.load(Ordering::SeqCst);
        let title: &String = &format!("NiggaBot | Slat.cc View Bot | Views Sent: {} | Failed Requests: {}", sent, failed);
        winconsole::console::set_title(title).unwrap();
    }
}

// main function
#[tokio::main]
async fn main() {
    let _enable_ansi: Result<(), u32> = ansi_term::enable_ansi_support();

    let proxies: Arc<Vec<String>> = match read_proxies_from_file("proxies.txt") {
        Ok(proxies) => proxies,
        Err(_) => {
            return;
        }
    };

    println!("{}", Purple.paint("What is your slat.cc user id?"));
    let mut user_id_input: String = String::new();
    io::stdin().read_line(&mut user_id_input).expect("Failed to read input...");
    let user_id: u32 = match user_id_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            return;
        }
    };

    winconsole::console::clear().ok();

    let num_of_threads: usize = 5000;
    let stats: Arc<Stats> = Arc::new(Stats {
        views_sent: AtomicU32::new(0),
        views_failed: AtomicU32::new(0),
    });
    let stats_clone: Arc<Stats> = Arc::clone(&stats);

    tokio::spawn(async move {
        update_title(stats_clone).await;
    });

    let mut handles: Vec<tokio::task::JoinHandle<()>> = vec![];
    for _ in 0..num_of_threads {
        let user_id: u32 = user_id;
        let proxies: Arc<Vec<String>> = Arc::clone(&proxies);
        let stats: Arc<Stats> = stats.clone();
        let handle: tokio::task::JoinHandle<()> = tokio::spawn(async move {
            loop {
                send_slat_request(user_id, proxies.clone(), stats.clone()).await;
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.await.unwrap();
    }
}