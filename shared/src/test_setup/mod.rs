use std::fmt;
use std::sync::{Arc, Mutex};

// Memory Writer that writes logs into an Vec<u8>
pub struct MemoryWriterFactory {
    buffer: Arc<Mutex<Vec<u8>>>,
}

impl fmt::Write for MemoryWriterFactory {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let mut buffer = self.buffer.lock().unwrap();
        buffer.extend_from_slice(s.as_bytes());
        Ok(())
    }
}

async fn idb_signal_basic_reactivity() {
    // Your test code here
}
