use std::sync::{Arc, Mutex};

use tracing_subscriber::fmt::MakeWriter;
use wasm_bindgen_test::console_log;

pub struct InMemoryWriter {}

impl std::io::Write for InMemoryWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let buf_str = String::from_utf8(buf.to_vec()).unwrap();
        console_log!("{}", buf_str);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

pub struct MemoryWriterFactory {
    pub buffer: Arc<Mutex<Vec<u8>>>,
}

impl<'a> MakeWriter<'a> for MemoryWriterFactory {
    type Writer = InMemoryWriter;

    fn make_writer(&'a self) -> Self::Writer {
        InMemoryWriter {}
    }
}
