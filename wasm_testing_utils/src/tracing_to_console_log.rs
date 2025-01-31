pub struct MemoryWriterFactory {
    pub buffer: std::sync::Arc<std::sync::Mutex<Vec<u8>>>,
}

impl std::io::Write for MemoryWriterFactory {
    fn write(&mut self, buf: &[u8]) -> Result<usize, std::io::Error> {
        let mut buffer = self.buffer.lock().unwrap();
        buffer.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<(), std::io::Error> {
        Ok(())
    }
}
