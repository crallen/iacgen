use anyhow::Result;

use crate::core::OutputWriter;

pub struct StdoutWriter;

impl OutputWriter for StdoutWriter {
    fn write(&mut self, content: &str) -> Result<()> {
        print!("{}", content);
        Ok(())
    }
}
