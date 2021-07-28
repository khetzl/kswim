#[derive(Debug)]
pub enum PingResult {
    Pong, // Success
    Pang, // Timeout or Error
}
