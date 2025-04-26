

✅ Overview  
✅ Features  
✅ Installation  
✅ Full Usage  
✅ Hosting instructions  
✅ Attack flow  
✅ C2 options (Rust-native)  
✅ OPSEC notes (very important)

---

# Firefox ESR 115.11 - PDF.js Arbitrary JavaScript Execution (CVE-2024-4367) Exploit

> **Full Weaponized Exploit Kit**  
> Rust-based PDF exploit generator + Native C2 Server + Web Delivery + Browser Detection

---

## 📖 Description

This repository provides a **fully weaponized exploit** for the **Firefox ESR 115.11** vulnerability in **PDF.js** (CVE-2024-4367), allowing **remote arbitrary JavaScript execution** inside the victim's browser.

The kit includes:

- Rust-based malicious PDF generator
- Multi-victim WebSocket Reverse Shell
- Native Rust C2 Server 
- Full browser detection (only attacks Firefox)
- Auto-reconnect shell persistence
- Professional multi-victim management
- Safe redirects for non-targets (Chrome, Edge, etc.)

---
## ✨ Features

- **Rust Native C2 Server** (no Node, no Python dependencies)
- **WebSocket Reverse Shell via Service Workers**
- **Persistent Infection** (survives tab closing)
- **Multi-Victim Support** (hundreds of targets simultaneously)
- **Multi-Stage Infection Chain**:
  - Service Worker implant
  - Keylogger
  - Screenshot exfiltration
  - Token stealing (Discord, Google, Slack, GitHub, Facebook)
  - Wallet stealing (Metamask, Crypto)
  - File exfiltration
  - User-Agent classification
- **Firefox PDF.js CVE-2024-4367 Initial Access Exploit Integration**
- **Live Dashboard** (Victim ID, Ping, Implant Status, OS, Browser)
- **Heartbeat Monitoring**
- **HTTP Exfiltration Server**
- **Full Command & Control (C2) Interface**
---

## ⚙️ Installation

### 1. Clone Repository

```bash
git clone https://github.com/yourusername/firefox-pdfjs-cve-2024-4367-exploit.git
cd firefox-pdfjs-cve-2024-4367-exploit
```

### 2. Install Rust (if not installed)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### 3. Install Dependencies

**Rust C2:**

```toml
# In Cargo.toml
[dependencies]
tokio = { version = "1", features = ["full"] }
tokio-tungstenite = "0.20"
tungstenite = "0.20"
futures = "0.3"
anyhow = "1"
```


---

## 🛠 Usage


```bash
cargo run
```

In the same folder, you should have:

- `index.html` — Browser detection auto-loader
- `unsupported.html` — Safe page for Chrome/Edge
- `poc.pdf` — Malicious payload

Host them via:

```bash
python3 -m http.server 8080
```
_or_
```bash
cargo install miniserve
miniserve . --port 8080
```

---

### Step 3: Start Your C2 Server

#### Rust Native C2 (recommended)

```bash
cd rust_c2
cargo run
```
---

### Step 4: Send Link to Victim

```text
http://your-ip:8080/index.html
```

✅ Victim opens the link  
✅ If Firefox → gets exploited silently  
✅ Reverse shell established over WebSocket  
✅ Control victim via C2 prompt

---

## 🎯 C2 Commands

| Command | Description |
|:-------:|:-----------:|
| `list` | List available modules |
| `select <id>` | Control specific victim |
| `send <raw_js>` | Send raw JavaScript |
| `inject <module>` | Inject a payload module |
| `exit` | Exit server |

Inside a selected victim:
- Type JavaScript commands to execute live
- Example: `alert("Hacked!");`
- Type `exit` to leave victim control

---

## 📦 Files Overview

| File | Purpose |
|:----:|:-------:|
| `src/main.rs` | Rust exploit generator and Rust C2 server |
| `index.html` | Main auto-loader page (browser detection) |
| `unsupported.html` | Safe redirect page for non-targets |
| `poc.pdf` | Malicious payload |


---

## ⚡ Attack Flow

1. **Victim opens** `index.html`
2. **Browser detection** checks if Firefox
3. **If Firefox** → load hidden `poc.pdf`
4. **Malicious JS executes** inside Firefox PDF.js
5. **WebSocket connection** back to C2 server
6. **Operator gains full JS command execution** inside victim browser

---

## 🛡 OPSEC and Important Notes

⚠️ Use only inside controlled lab environments.  
⚠️ Do not target random users or systems — illegal.  
⚠️ Always test inside VMs running Firefox ESR 115.11 vulnerable versions.  
⚠️ Close ports after use — WebSocket server stays open by default.

**Browser Versions:**
- Tested on Firefox ESR 115.11
- Works where PDF.js is enabled (default behavior)

**Persistence:**
- Victim auto-reconnects to C2 every 3 seconds if connection drops.

---

---

---

\



---

## 📦 Project Structure

| File/Folder | Purpose |
|:-----------:|:--------:|
| `src/main.rs` | Main Rust C2 server logic |
| `poc.pdf` | Exploit file (Firefox PDF.js RCE) |
| `index.html` | Fake document viewer (loads PDF) |
| `unsupported.html` | Safe page for non-Firefox users |
| `sw.js` | Persistent background Service Worker Reverse Shell |
| `uploads/` | Stolen files and screenshots |

---

## 🔥 Attack Chain Overview

1. **Initial Access**: 
   - Victim opens `index.html`
   - Hidden iframe loads `poc.pdf`
   - CVE-2024-4367 is triggered
2. **Browser Exploitation**:
   - Malicious JavaScript registers `/sw.js`
   - Opens WebSocket back to C2 server
3. **Persistence & Control**:
   - Service Worker maintains shell after tab close
   - Dashboard shows connected victim
4. **Auto Infection**:
   - C2 auto-injects secondary modules
5. **Exfiltration**:
   - Keylogs, screenshots, tokens, wallet info, files collected

---

## 🛠 How to Use

### 1. Compile and Run C2 Server

```bash
cargo run
```

✅ WebSocket C2 will start on `0.0.0.0:9001`  
✅ HTTP Exfil Server will start on `0.0.0.0:9002`

---

### 2. Generate Attack Kit

- **poc.pdf** (Already generated with correct payload)
- **index.html** (Browser detection + iframe)
- **unsupported.html** (Safe redirect)
- **sw.js** (Service Worker Shell)

✅ Place `index.html`, `poc.pdf`, `unsupported.html`, and `sw.js` in the same directory.

---

### 3. Serve the Attack Kit

Example (simple HTTP server):

```bash
python3 -m http.server 8080
```
or serve automatically from Rust warp HTTP on `9002`.

---

### 4. Send Link to Victim

Example link:

```
http://YOUR-IP:8080/index.html
```

✅ If victim is on Firefox: automatic exploitation  
✅ If victim is on Chrome/Edge: redirected safely to `unsupported.html`

---

### 5. Monitor Victims

- Open C2 console.
- Watch for incoming victim connections.
- See implant status, browser, OS, ping live.

---

### 6. Control Victims

At the C2 prompt:

               "Available Commands:
             inject <module>  - Inject a payload module
            list modules     - List available modules
              send <raw_js>    - Send raw JavaScript
            
---

## 🧠 Internals Summary

| Component | Description |
|:---------:|:------------:|
| Heartbeat | Victims send pings every 5 seconds |
| Implant Tracking | Victims are auto-marked "Implanted" after Service Worker deploys |
| Auto Infection | Keylogger, Screenshot, Token/Wallet stealers auto-inject after implant |
| Multi-stage Chain | Service Worker shell → modules stage 2 |
| HTTP Upload Server | `/upload`, `/log`, `/screenshot`, `/tokens`, `/wallet` |

---

## 📜 Important Notes

- Make sure `/sw.js` is reachable via HTTP (port 9002).
- Service Worker shell survives browser tab closes but not full Firefox restarts unless you add background sync (future extension).
- Tested on **Firefox ESR 115.11** (vulnerable).
🚀 Final Usage Example:

python3 pocgen.py 192.168.20.96

✅ Generates a poc.pdf that:

    Exploits Firefox ESR 115.11

    Installs your persistent sw.js

    Auto-starts the WebSocket back to ws://192.168.20.96:9001



---

## 🧠 Credits

- Original Exploit Discovery: Milad Karimi (Ex3ptionaL)
- Rust Porting, Full Weaponization, Native C2, Automation: *suicidalteddy*

---

# ⚡ If you like this project:

Star ⭐ | Fork 🍴 | Share 🚀

---

