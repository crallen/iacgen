use anyhow::Result;
use std::path::PathBuf;

use crate::core::OutputWriter;

pub struct FileWriter {
    path: PathBuf,
    buffer: String,
}

impl FileWriter {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            buffer: String::new(),
        }
    }
}

impl OutputWriter for FileWriter {
    fn write(&mut self, content: &str) -> Result<()> {
        self.buffer.push_str(content);
        Ok(())
    }

    fn flush(&mut self) -> Result<()> {
        std::fs::write(&self.path, &self.buffer)?;
        self.buffer.clear();
        Ok(())
    }
}
