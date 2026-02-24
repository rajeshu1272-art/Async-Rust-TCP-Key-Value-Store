# Async Rust TCP Key-Value Store

A simple high-performance asynchronous TCP key-value server built in Rust using Tokio.  
This project demonstrates systems programming concepts such as concurrency, networking, and memory safety.

---

## 🚀 Features

- Async TCP server using Tokio runtime  
- Concurrent client handling using async tasks  
- In-memory key-value storage using HashMap  
- Thread-safe shared state using Arc and Mutex  
- Simple text-based protocol supporting SET and GET commands  

---

## 🧠 Architecture Overview

- TCP server listens on port 6379  
- Each client connection is handled in a separate async task  
- Shared HashMap is protected using Mutex and Arc for thread safety  
- Commands are parsed and executed in real-time  

---

## 🛠️ Tech Stack

- Rust  
- Tokio (async runtime)  
- TCP networking  
- HashMap  
- Arc & Mutex for concurrency  

---

## ▶️ How to Run

### 1. Clone the repository
```bash
git clone <your-repo-url>
cd rust_kv_store

