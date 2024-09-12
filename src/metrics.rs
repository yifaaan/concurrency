use std::collections::HashMap;
use std::fmt::Display;
use std::sync::Arc;
use std::sync::RwLock;

#[derive(Debug, Clone)]
pub struct Metrics {
    data: Arc<RwLock<HashMap<String, i64>>>,
}

impl Metrics {
    pub fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn inc(&self, key: impl Into<String>) -> anyhow::Result<()> {
        self.data
            .write()
            .map_err(|e| anyhow::anyhow!("Unlock error: {}", e))?
            .entry(key.into())
            .and_modify(|e| *e += 1)
            .or_insert(0);
        Ok(())
    }

    pub fn dec(&self, key: impl Into<String>) -> anyhow::Result<()> {
        self.data
            .write()
            .map_err(|e| anyhow::anyhow!("Unlock error: {}", e))?
            .entry(key.into())
            .and_modify(|e| *e -= 1)
            .or_insert(0);
        Ok(())
    }

    pub fn snapshot(&self) -> anyhow::Result<HashMap<String, i64>> {
        Ok(self
            .data
            .write()
            .map_err(|e| anyhow::anyhow!("Unlock error: {}", e))?
            .clone())
    }
}

impl Display for Metrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // write!(f, "{")?;
        let data = self.data.read().map_err(|_| std::fmt::Error {})?;

        for (key, value) in data.iter() {
            write!(f, "{}: {},", key, value)?;
        }
        write!(f, "\n")?;
        Ok(())
    }
}
