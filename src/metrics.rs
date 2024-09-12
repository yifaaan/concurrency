use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Debug, Clone)]
pub struct Metrics {
    data: Arc<Mutex<HashMap<String, i64>>>,
}

impl Metrics {
    pub fn new() -> Self {
        Self {
            data: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn inc(&self, key: impl Into<String>) -> anyhow::Result<()> {
        self.data
            .lock()
            .map_err(|e| anyhow::anyhow!("Unlock error: {}", e))?
            .entry(key.into())
            .and_modify(|e| *e += 1)
            .or_insert(0);
        Ok(())
    }

    pub fn dec(&self, key: impl Into<String>) -> anyhow::Result<()> {
        self.data
            .lock()
            .map_err(|e| anyhow::anyhow!("Unlock error: {}", e))?
            .entry(key.into())
            .and_modify(|e| *e -= 1)
            .or_insert(0);
        Ok(())
    }

    pub fn snapshot(&self) -> anyhow::Result<HashMap<String, i64>> {
        Ok(self
            .data
            .lock()
            .map_err(|e| anyhow::anyhow!("Unlock error: {}", e))?
            .clone())
    }
}
