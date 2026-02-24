# Async-Rust-TCP-Key-Value-Store
- TCP server listens on port 6379   - Each client connection is handled in a separate async task   - Shared HashMap is protected using Mutex and Arc for thread safety   - Commands are parsed and executed in real-time 
